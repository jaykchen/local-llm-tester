use crate::utils::*;
use anyhow::anyhow;
use chrono::{DateTime, Duration, NaiveDate, Utc};
use derivative::Derivative;
use http_req::response::{self, Response};
use octocrab::models::{issues::Comment, issues::Issue, Repository};
use octocrab::Octocrab;

use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Derivative, Serialize, Deserialize, Debug, Clone)]
pub struct GitMemory {
    pub memory_type: MemoryType,
    #[derivative(Default(value = "String::from(\"\")"))]
    pub name: String,
    #[derivative(Default(value = "String::from(\"\")"))]
    pub tag_line: String,
    #[derivative(Default(value = "String::from(\"\")"))]
    pub source_url: String,
    #[derivative(Default(value = "String::from(\"\")"))]
    pub payload: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MemoryType {
    Commit,
    Issue,
    Discussion,
    Meta,
}

pub async fn get_community_profile_data(owner: &str, repo: &str) -> Option<String> {
    #[derive(Deserialize, Debug)]
    struct CommunityProfile {
        description: String,
        // documentation: Option<String>,
    }

    let community_profile_url =
        format!("https://api.github.com/repos/{owner}/{repo}/community/profile");

    let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required");
    let octocrab = Octocrab::builder()
        .personal_token(token)
        .build()
        .expect("octocrab failed to build");

    match octocrab
        .get::<CommunityProfile, _, ()>(&community_profile_url, None::<&()>)
        .await
    {
        Ok(profile) => {
            return Some(format!("Description: {}", profile.description));
        }
        Err(e) => println!("Error parsing Community Profile: {:?}", e),
    }
    None
}

pub async fn get_contributors(owner: &str, repo: &str) -> Result<Vec<String>, octocrab::Error> {
    #[derive(Debug, Deserialize)]
    struct GithubUser {
        login: String,
    }
    let mut contributors = Vec::new();
    let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required");
    let octocrab = Octocrab::builder()
        .personal_token(token)
        .build()
        .expect("octocrab failed to build");

    'outer: for n in 1..50 {
        println!("contributors loop {}", n);

        let contributors_route = format!(
            "https://api.github.com/repos/{owner}/{repo}/contributors?per_page=100&page={n}"
        );

        match octocrab
            .get::<Vec<GithubUser>, _, ()>(&contributors_route, None::<&()>)
            .await
        {
            Ok(user_vec) => {
                if user_vec.is_empty() {
                    break 'outer;
                }
                for user in &user_vec {
                    contributors.push(user.login.clone());
                    // println!("user: {}", user.login);
                    // upload_airtable(&user.login, "email", "twitter_username", false).await;
                }
            }

            Err(_e) => {
                println!("looping stopped: {:?}", _e);
                break 'outer;
            }
        }
    }

    Ok(contributors)
}

pub async fn get_readme(owner: &str, repo: &str) -> Option<String> {
    #[derive(Deserialize, Debug)]
    struct GithubReadme {
        content: Option<String>,
    }

    let readme_url = format!("https://api.github.com/repos/{owner}/{repo}/readme");

    let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required");
    let octocrab = Octocrab::builder()
        .personal_token(token)
        .build()
        .expect("octocrab failed to build");

    match octocrab
        .get::<GithubReadme, _, ()>(&readme_url, None::<&()>)
        .await
    {
        Ok(readme) => {
            if let Some(c) = readme.content {
                let cleaned_content = c.replace("\n", "");
                match base64::decode(&cleaned_content) {
                    Ok(decoded_content) => match String::from_utf8(decoded_content) {
                        Ok(out) => {
                            return Some(format!("Readme: {}", out));
                        }
                        Err(e) => {
                            println!("Failed to convert cleaned readme to String: {:?}", e);
                            return None;
                        }
                    },
                    Err(e) => {
                        println!("Error decoding base64 content: {:?}", e);
                        None
                    }
                }
            } else {
                println!("Content field in readme is null.");
                None
            }
        }
        Err(e) => {
            println!("Error parsing Readme: {:?}", e);
            None
        }
    }
}

pub async fn get_issues_in_range(
    owner: &str,
    repo: &str,
    user_name: Option<String>,
    range: u16,
    token: Option<String>,
) -> Option<(usize, Vec<Issue>)> {
    #[derive(Debug, Deserialize)]
    struct Page<T> {
        pub items: Vec<T>,
        pub total_count: Option<u64>,
    }

    let n_days_ago = (Utc::now() - Duration::days(range as i64))
        .format("%Y-%m-%dT%H:%M:%SZ")
        .to_string();

    let user_str = user_name.map_or(String::new(), |u| format!("involves:{}", u));

    let query = format!("repo:{owner}/{repo} is:issue {user_str} updated:>{n_days_ago}");
    let encoded_query = urlencoding::encode(&query);
    let token_str = match token {
        None => String::new(),
        Some(t) => format!("&token={}", t.as_str()),
    };
    let url_str = format!(
        "https://api.github.com/search/issues?q={}&sort=updated&order=desc&per_page=100{token_str}",
        encoded_query
    );
    // let url_str = format!(
    //     "https://api.github.com/search/issues?q={}&sort=updated&order=desc&per_page=100{token_str}",
    //     encoded_query
    // );

    let mut issue_vec = vec![];
    let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required");
    let octocrab = Octocrab::builder()
        .personal_token(token)
        .build()
        .expect("octocrab failed to build");

    match octocrab
        .get::<Page<Issue>, _, ()>(&url_str, None::<&()>)
        .await
    {
        Err(e) => {
            println!("error: {:?}", e);
        }
        Ok(issue_page) => {
            issue_vec = issue_page.items;
        }
    }
    let count = issue_vec.len();
    Some((count, issue_vec))
}

pub async fn get_issue_texts(issue: &Issue) -> Option<String> {
    let issue_creator_name = &issue.user.login;
    let issue_title = &issue.title;
    let issue_body = match &issue.body {
        Some(body) => squeeze_fit_remove_quoted(body, 500, 0.6),
        None => "".to_string(),
    };
    let issue_url = &issue.url.to_string();

    let labels = issue
        .labels
        .iter()
        .map(|lab| lab.name.clone())
        .collect::<Vec<String>>()
        .join(", ");

    let mut all_text_from_issue = format!(
        "User '{}', opened an issue titled '{}', labeled '{}', with the following post: '{}'.",
        issue_creator_name, issue_title, labels, issue_body
    );

    let mut current_page = 1;
    loop {
        let url_str = format!("{}/comments?&page={}", issue_url, current_page);

        let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required");
        let octocrab = Octocrab::builder()
            .personal_token(token)
            .build()
            .expect("octocrab failed to build");

        match octocrab
            .get::<Vec<Comment>, _, ()>(&url_str, None::<&()>)
            .await
        {
            Err(_e) => {
                println!(
                    "Error parsing Vec<Comment> at page {}: {:?}",
                    current_page, _e
                );
                break;
            }
            Ok(comments_obj) => {
                if comments_obj.is_empty() {
                    break;
                }
                for comment in &comments_obj {
                    let comment_body = match &comment.body {
                        Some(body) => squeeze_fit_remove_quoted(body, 300, 0.6),
                        None => "".to_string(),
                    };
                    let commenter = &comment.user.login;
                    let commenter_input = format!("{} commented: {}", commenter, comment_body);
                    if all_text_from_issue.len() > 45_000 {
                        break;
                    }
                    all_text_from_issue.push_str(&commenter_input);
                }
            }
        }

        current_page += 1;
    }

    Some(all_text_from_issue)
}

pub async fn get_commits_in_range_search(
    owner: &str,
    repo: &str,
    user_name: Option<String>,
    range: u16,
) -> Option<(usize, Vec<GitMemory>)> {
    #[derive(Debug, Deserialize)]
    struct Page<T> {
        pub items: Vec<T>,
        pub total_count: Option<u64>,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    struct User {
        login: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct GithubCommit {
        sha: String,
        html_url: String,
        author: Option<User>, // made nullable
        commit: CommitDetails,
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct CommitDetails {
        message: String,
        // committer: CommitUserDetails,
    }

    let author_str = match &user_name {
        None => String::from(""),
        Some(t) => format!("author:{}", t.as_str()),
    };
    let now = Utc::now();
    let n_days_ago = (now - Duration::days(range as i64)).date_naive();

    let query = format!(
        "repo:{}/{} {} committer-date:>{}",
        owner, repo, author_str, n_days_ago
    );

    // let query = format!("repo:{}/{}{}", owner, repo, author_str);
    let encoded_query = urlencoding::encode(&query);
    let mut git_memory_vec = vec![];
    let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required");
    let octocrab = Octocrab::builder()
        .personal_token(token)
        .build()
        .expect("octocrab failed to build");

    let url_str = format!(
        "https://api.github.com/search/commits?q={}&sort=committer-date&order=desc&per_page=100",
        encoded_query
    );
    // println!("url_str: {}", url_str);

    // let url_str = format!(
    //     "https://api.github.com/repos/wasmedge/wasmedge/commits?q=author:hydai%20committer-date:>2024-01-04&author=hydai&sort=desc&per_page=100"
    // );
    use hyper::http::{HeaderMap, HeaderValue, Method, Uri};

    match octocrab
        .get::<Page<GithubCommit>, _, ()>(&url_str, None::<&()>)
        .await
    {
        Err(e) => {
            println!("Error parsing commits: {:?}", e);
        }
        Ok(commits_page) => {
            for commit in &commits_page.items {
                if let Some(author) = &commit.author {
                    // println!("commit url: {:?}", &commit.html_url.clone());
                    git_memory_vec.push(GitMemory {
                        memory_type: MemoryType::Commit,
                        name: author.login.clone(),
                        tag_line: commit.commit.message.clone(),
                        source_url: commit.html_url.clone(),
                        payload: String::from(""),
                    });
                }
            }
            if &commits_page.items.len() < &100 {}
        }
    }
    let count = git_memory_vec.len();

    Some((count, git_memory_vec))
}
