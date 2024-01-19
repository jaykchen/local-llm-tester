use crate::github_data_fetchers::*;
use crate::utils::*;
// use async_openai::types::FinishReason;
use chrono::{ DateTime, Utc };
use log;
use octocrab::models::{ issues::Comment, issues::Issue };
use octocrab::Octocrab;
use serde::Deserialize;
use std::collections::{ HashMap, HashSet };

pub async fn is_valid_owner_repo(
    owner: &str,
    repo: &str
) -> anyhow::Result<(String, String, HashSet<String>)> {
    #[derive(Deserialize)]
    struct CommunityProfile {
        description: Option<String>,
        files: FileDetails,
        updated_at: Option<DateTime<Utc>>,
    }
    #[derive(Debug, Deserialize)]
    pub struct FileDetails {
        readme: Option<Readme>,
    }
    #[derive(Debug, Deserialize)]
    pub struct Readme {
        url: Option<String>,
    }
    let community_profile_url = format!(
        "https://api.github.com/repos/{}/{}/community/profile",
        owner,
        repo
    );

    let description;
    let mut has_readme = false;
    let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required");
    let octocrab = Octocrab::builder()
        .personal_token(token)
        .build()
        .expect("octocrab failed to build");

    match octocrab.get::<CommunityProfile, _, ()>(&community_profile_url, None::<&()>).await {
        Ok(profile) => {
            description = profile.description.as_ref().unwrap_or(&String::from("")).to_string();

            has_readme = profile.files.readme
                .as_ref()
                .unwrap_or(&(Readme { url: None }))
                .url.is_some();
        }
        Err(e) => {
            println!("Error parsing Community Profile: {:?}", e);
            return Err(anyhow::anyhow!("no Community Profile, so invalid owner/repo: {:?}", e));
        }
    }

    let mut payload = String::new();

    if has_readme {
        if let Some(content) = get_readme(owner, repo).await {
            let content = content.chars().take(20000).collect::<String>();
            match analyze_readme(&content).await {
                Some(summary) => {
                    payload = summary;
                }
                None => println!("Error parsing README.md: {}/{}", owner, repo),
            }
        }
    }

    if payload.is_empty() {
        payload = description.clone();
    }

    let contributors_set = match get_contributors(owner, repo).await {
        Ok(contributors) => contributors.into_iter().collect::<HashSet<String>>(),
        Err(_e) => HashSet::<String>::new(),
    };

    Ok((format!("{}/{}", owner, repo), payload, contributors_set))
}

pub async fn analyze_readme(content: &str) -> Option<String> {
    let sys_prompt_1 = &format!(
        "Your task is to objectively analyze a GitHub profile and the README of their project. Focus on extracting factual information about the features of the project, and its stated objectives. Avoid making judgments or inferring subjective value."
    );

    let content = if content.len() > 48_000 {
        squeeze_fit_remove_quoted(&content, 9_000, 0.7)
    } else {
        content.to_string()
    };
    let usr_prompt_1 = &format!(
        "Based on the profile and README provided: {content}, extract a concise summary detailing this project's factual significance in its domain, their areas of expertise, and the main features and goals of the project. Ensure the insights are objective and under 110 tokens."
    );

    match chat_inner_async(sys_prompt_1, usr_prompt_1, 256, "mistralai/Mistral-7B-Instruct-v0.1").await {
        Ok(r) => {
            return Some(r);
        }
        Err(e) => {
            println!("Error summarizing meta data: {}", e);
            None
        }
    }
}

pub async fn process_issues(
    inp_vec: Vec<Issue>,
    target_person: Option<String>,
    contributors_set: HashSet<String>,
    token: Option<String>
) -> anyhow::Result<HashMap<String, (String, String)>> {
    use futures::future::join_all;

    let issue_futures: Vec<_> = inp_vec
        .into_iter()
        .map(|issue| {
            let target_person = target_person.clone();
            let token = token.clone();
            let contributors_set = contributors_set.clone();
            async move {
                let ve = analyze_issue_integrated(
                    &issue,
                    target_person,
                    contributors_set,
                    token
                ).await.ok()?;
                Some(ve)
            }
        })
        .collect();

    let results = join_all(issue_futures).await;
    let mut issues_map = HashMap::<String, (String, String)>::new();

    for result in results.into_iter().flatten() {
        for item in result {
            let (user_name, url, summary) = item;
            // println!(
            //     "User: {:?}, Url: {:?}, Summary: {:?}",
            //     user_name.clone(),
            //     url.clone(),
            //     summary.clone()
            // );
            issues_map
                .entry(user_name.clone())
                .and_modify(|tup| {
                    tup.0.push_str("\n");
                    tup.0.push_str(&url);
                    tup.1.push_str("\n");
                    tup.1.push_str(&summary);
                })
                .or_insert((url.to_string(), summary.to_string()));
        }
    }

    if issues_map.len() == 0 {
        anyhow::bail!("No issues processed");
    }

    Ok(issues_map)
}

pub async fn analyze_issue_integrated(
    issue: &Issue,
    target_person: Option<String>,
    contributors_set: HashSet<String>,
    token: Option<String>
) -> anyhow::Result<Vec<(String, String, String)>> {
    let issue_creator_name = &issue.user.login;
    let issue_title = issue.title.to_string();
    let issue_number = issue.number;
    let mut issue_commenters_to_watch = Vec::new();
    let issue_body = match &issue.body {
        Some(body) => squeeze_fit_remove_quoted(body, 400, 0.7),
        None => "".to_string(),
    };
    let issue_url = issue.url.to_string();
    let source_url = issue.html_url.to_string();

    let labels = issue.labels
        .iter()
        .map(|lab| lab.name.clone())
        .collect::<Vec<String>>()
        .join(", ");

    let mut all_text_from_issue = format!(
        "User '{}', opened an issue titled '{}', labeled '{}', with the following post: '{}'.",
        issue_creator_name,
        issue_title,
        labels,
        issue_body
    );

    let token_str = match token {
        None => String::new(),
        Some(t) => format!("&token={}", t.as_str()),
    };

    // let comments_url = format!(
    //     "{}/comments?sort=updated&order=desc&per_page=100{}",
    //     issue_url.replace("https://api.github.com/", ""),
    //     token_str
    // );
    let comments_url = format!(
        "{}/comments?sort=updated&order=desc&per_page=100{}",
        issue_url,
        token_str
    );
    // let octocrab = get_octo(&GithubLogin::Default);

    let response = github_http_get(&comments_url).await?;
    let comments_obj = serde_json::from_slice::<Vec<Comment>>(&response)?;

    for comment in &comments_obj {
        let comment_body = match &comment.body {
            Some(body) => squeeze_fit_remove_quoted(body, 200, 1.0),
            None => String::new(),
        };
        let commenter = &comment.user.login;
        if contributors_set.contains(commenter) {
            issue_commenters_to_watch.push(commenter.to_string());
        }

        let commenter_input = format!("{} commented: {}", commenter, comment_body);
        all_text_from_issue.push_str(&commenter_input);
    }

    all_text_from_issue = all_text_from_issue.chars().take(32_000).collect();

    let target_str = target_person
        .clone()
        .map_or("key participants".to_string(), |t| t.to_string());

    let sys_prompt_1 = &format!(
        "Given the information that user '{issue_creator_name}' opened an issue titled '{issue_title}', your task is to deeply analyze the content of the issue posts. Distill the crux of the issue, the potential solutions suggested, and evaluate the significant contributions of the participants in resolving or progressing the discussion."
    );

    let commenters_to_watch_str = if !target_str.is_empty() || issue_commenters_to_watch.len() == 0 {
        target_str
    } else {
        issue_commenters_to_watch.join(", ")
    };

    // let usr_prompt_1 = &format!(
    //     "Analyze the GitHub issue content: {}. Provide a concise analysis touching upon: The central problem discussed in the issue. The main solutions proposed or agreed upon. Highlight the role and significance of '{}' in contributing towards the resolution or progression of the discussion. Format the analysis into a flat JSON structure with one level of depth where each key maps directly to a single string value. Use the following template, replacing 'contributor_name' with the actual contributor's name, and 'summary' with your analysis of their contributions:
    //     {{
    //     \"contributor_name_1\": \"summary\",
    //     \"contributor_name_2\": \"summary\"
    //     }}",
    //     all_text_from_issue,
    //     commenters_to_watch_str
    // );
    let usr_prompt_1 = &format!(
        "Analyze the GitHub issue content: {}. Provide a concise analysis touching upon: The central problem discussed in the issue. The main solutions proposed or agreed upon. Highlight the role and significance of '{}' in contributing towards the resolution or progression of the discussion. If the target person's contribution is negligible or non-existent, leave the corresponding summary blank. Format the analysis into a flat JSON structure with one level of depth where each key maps directly to a single string value. Use the following template, replacing 'contributor_name' with the actual contributor's name, and 'summary' with your analysis of their contributions or an empty string if their contribution is negligible: 
        {{ 
        \"contributor_name_1\": \"summary\",
        \"contributor_name_2\": \"summary\"
        }}",
        all_text_from_issue,
        commenters_to_watch_str
    );

    match chat_inner_async(sys_prompt_1, usr_prompt_1, 128, "mistralai/Mistral-7B-Instruct-v0.1").await {
        Ok(r) => {
            let parsed = parse_issue_summary_from_json(&r)
                .ok()
                .unwrap_or_else(|| vec![]);

            let out = parsed
                .into_iter()
                .map(|(user_name, summary)| { (user_name, source_url.clone(), summary) })
                .collect::<Vec<(String, String, String)>>();

            Ok(out)

            // let out = format!("{} {}", issue_url, r);
            // let name = target_person.map_or(issue_creator_name.to_string(), |t| t.to_string());
            // let gm = GitMemory {
            //     memory_type: MemoryType::Issue,
            //     name: name,
            //     tag_line: issue_title,
            //     source_url: source_url,
            //     payload: r,
            // };

            // Ok((out, gm))
        }
        Err(_e) => {
            println!("Error generating issue summary #{}: {}", issue_number, _e);
            Err(anyhow::anyhow!("Error generating issue summary #{}: {}", issue_number, _e))
        }
    }
}

pub async fn analyze_commit_integrated(
    user_name: &str,
    tag_line: &str,
    url: &str,
    _turbo: bool,
    is_sparce: bool,
    token: Option<String>
) -> anyhow::Result<String> {
    use reqwest::header::{ HeaderMap, HeaderValue, AUTHORIZATION, USER_AGENT };
    let token_str = match token {
        None => String::new(),
        Some(t) => format!("&token={}", t.as_str()),
    };

    let commit_patch_str = format!("{url}.patch{token_str}");
    // let url = Url::try_from(commit_patch_str.as_str())
    //     .expect(&format!("Error generating URI from {:?}", commit_patch_str));

    let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required");
    // let octocrab = Octocrab::builder()
    //     .personal_token(token)
    //     .build()
    //     .expect("octocrab failed to build");
    let client = reqwest::Client::new();

    // Create a header map and add the `Authorization` and `User-Agent` headers
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&format!("token {}", token))?);
    // headers.insert(
    //     USER_AGENT,
    //     HeaderValue::from_static("my-awesome-github-app"),
    // );

    // Send the GET request with the headers
    let response = client.get(commit_patch_str).headers(headers).send().await?;

    // Check the response status code
    if !response.status().is_success() {
        return Err(anyhow::anyhow!("Request failed with status: {}", response.status()));
    }

    // Read the response text
    let text = response
        .text().await
        .map_err(|e| anyhow::anyhow!("Failed to read response text: {}", e))?;

    // println!("{:?}", text.clone()); // let mut stripped_texts = String::with_capacity(text.len());

    // 'commit_text_block: {
    //     let lines_count = text.lines().count();
    //     if lines_count > 150 {
    //         stripped_texts = text
    //             .splitn(2, "diff --git")
    //             .nth(0)
    //             .unwrap_or("")
    //             .to_string();
    //         break 'commit_text_block;
    //     }

    //     let mut inside_diff_block = false;

    //     match is_sparce {
    //         false => {
    //             for line in text.lines() {
    //                 if line.starts_with("diff --git") {
    //                     inside_diff_block = true;
    //                     stripped_texts.push_str(line);
    //                     stripped_texts.push('\n');
    //                     continue;
    //                 }

    //                 if inside_diff_block {
    //                     if line
    //                         .chars()
    //                         .any(|ch| ch == '[' || ch == ']' || ch == '{' || ch == '}')
    //                     {
    //                         continue;
    //                     }
    //                 }

    //                 stripped_texts.push_str(line);
    //                 stripped_texts.push('\n');

    //                 if line.is_empty() {
    //                     inside_diff_block = false;
    //                 }
    //             }
    //         }
    //         true => stripped_texts = text.to_string(),
    //     }
    // }
    // slack_flows::send_message_to_channel("ik8", "ch_rep", stripped_texts.clone()).await;

    let sys_prompt_1 = &format!(
        "Given a commit patch from user {user_name}, analyze its content. Focus on changes that substantively alter code or functionality. A good analysis prioritizes the commit message for clues on intent and refrains from overstating the impact of minor changes. Aim to provide a balanced, fact-based representation that distinguishes between major and minor contributions to the project. Keep your analysis concise."
    );

    let stripped_texts = if !is_sparce {
        let stripped_texts = text.splitn(2, "diff --git").nth(0).unwrap_or("").to_string();

        let stripped_texts = squeeze_fit_remove_quoted(&stripped_texts, 5_000, 1.0);
        squeeze_fit_post_texts(&stripped_texts, 3_000, 0.6)
    } else {
        text.chars().take(24_000).collect::<String>()
    };

    // let stripped_texts = if turbo {
    //     squeeze_fit_post_texts(&stripped_texts, 3_000, 0.6)
    // } else {
    //     if stripped_texts.len() > 12_000 {
    //     }
    //     squeeze_fit_post_texts(&stripped_texts, 12_000, 0.6)
    // };

    let usr_prompt_1 = &format!(
        "Analyze the commit patch: {stripped_texts}, and its description: {tag_line}. Summarize the main changes, but only emphasize modifications that directly affect core functionality. A good summary is fact-based, derived primarily from the commit message, and avoids over-interpretation. It recognizes the difference between minor textual changes and substantial code adjustments. Conclude by evaluating the realistic impact of {user_name}'s contributions in this commit on the project. Limit the response to 110 tokens."
    );

    let sha_serial = match url.rsplitn(2, "/").nth(0) {
        Some(s) => s.chars().take(5).collect::<String>(),
        None => "0000".to_string(),
    };
    match chat_inner_async(sys_prompt_1, usr_prompt_1, 128, "mistralai/Mistral-7B-Instruct-v0.1").await {
        Ok(r) => {
            let out = format!("{} {}", url, r);
            Ok(out)
        }
        Err(_e) => {
            println!("Error generating issue summary #{}: {}", sha_serial, _e);
            Err(anyhow::anyhow!("Error generating issue summary #{}: {}", sha_serial, _e))
        }
    }
}

pub async fn process_commits(
    inp_vec: Vec<GitMemory>,
    commits_map: &mut HashMap<String, (String, String)>
) -> anyhow::Result<()> {
    use futures::future::join_all;

    let mut results = Vec::new();

    for commit_obj in inp_vec {
        let url = format!("{}.patch", commit_obj.source_url);
        let response = github_http_get(&url).await?;
        let text = String::from_utf8(response)?;
        let raw_len = text.clone().len();
        let stripped_texts = text.chars().take(24_000).collect::<String>();
        // let stripped_texts = String::from_utf8(response).ok()?.chars().take(24_000).collect::<String>();
        let user_name = commit_obj.name.clone();

        let tag_line = commit_obj.tag_line;

        let sys_prompt_1 = format!(
            "You're a GitHub data analysis bot. Analyze the following commit patch for its content and implications."
        );

        let usr_prompt_1 = format!(
            "Examine the commit patch: {stripped_texts}, and identify the key changes and their potential impact on the project. Exclude technical specifics, code excerpts, file changes, and metadata. Highlight only those changes that substantively alter code or functionality. Provide a fact-based representation that differentiates major from minor contributions. The commit is described as: '{tag_line}'. Summarize the main changes, focusing on modifications that directly affect core functionality, and provide insight into the value or improvement the commit brings to the project."
        );

        let usr_prompt_2 = format!(
            "Based on the analysis provided, craft a concise, non-technical summary of the key technical contributions made by {user_name} this week. The summary should be a full sentence or a short paragraph suitable for a general audience, emphasizing the contributions' significance to the project."
        );


        let summary = chain_of_chat(
            &sys_prompt_1,
            &usr_prompt_1,
            "commit-99",
            512,
            &usr_prompt_2,
            128,
            "chained-prompt-commit"
        ).await?;
        println!("Commit: {:?}\n len: {} Summary: {:?}", url, raw_len, summary.clone());
        results.push((commit_obj.name, commit_obj.source_url, summary));
    }

    for result in results {
        let (user_name, url, summary): (String, String, String) = result;
        commits_map
            .entry(user_name.clone())
            .and_modify(|tup| {
                tup.0.push_str("\n");
                tup.0.push_str(&url);
                tup.1.push_str("\n");
                tup.1.push_str(&summary);
            })
            .or_insert((url, summary.to_string()));
    }

    Ok(())
}
