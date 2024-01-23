use http_req::{request::Method, request::Request, uri::Uri};
// use log;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, USER_AGENT};
// use reqwest::{header, Client};
use secrecy::Secret;
use serde::Deserialize;
use std::collections::HashMap;

use async_openai::{
    config::Config,
    types::{
        // ChatCompletionFunctionsArgs, ChatCompletionRequestMessage,
        ChatCompletionRequestSystemMessageArgs,
        ChatCompletionRequestUserMessageArgs,
        // ChatCompletionTool, ChatCompletionToolArgs, ChatCompletionToolType,
        CreateChatCompletionRequestArgs,
    },
    Client as OpenAIClient,
};

pub fn squeeze_fit_remove_quoted(inp_str: &str, max_len: u16, split: f32) -> String {
    let mut body = String::new();
    let mut inside_quote = false;

    for line in inp_str.lines() {
        if line.contains("```") || line.contains("\"\"\"") {
            inside_quote = !inside_quote;
            continue;
        }

        if !inside_quote {
            let cleaned_line = line
                .split_whitespace()
                .filter(|word| word.len() < 150)
                .collect::<Vec<&str>>()
                .join(" ");
            body.push_str(&cleaned_line);
            body.push('\n');
        }
    }

    let body_words: Vec<&str> = body.split_whitespace().collect();
    let body_len = body_words.len();
    let n_take_from_beginning = ((body_len as f32) * split) as usize;
    let n_keep_till_end = body_len - n_take_from_beginning;

    // Range check for drain operation
    let drain_start = if n_take_from_beginning < body_len {
        n_take_from_beginning
    } else {
        body_len
    };

    let drain_end = if n_keep_till_end <= body_len {
        body_len - n_keep_till_end
    } else {
        0
    };

    let final_text = if body_len > (max_len as usize) {
        let mut body_text_vec = body_words.to_vec();
        body_text_vec.drain(drain_start..drain_end);
        body_text_vec.join(" ")
    } else {
        body
    };

    final_text
}

pub fn squeeze_fit_post_texts(inp_str: &str, max_len: u16, split: f32) -> String {
    let bpe = tiktoken_rs::cl100k_base().unwrap();

    let input_token_vec = bpe.encode_ordinary(inp_str);
    let input_len = input_token_vec.len();
    if input_len < (max_len as usize) {
        return inp_str.to_string();
    }
    // // Filter out the tokens corresponding to lines with undesired patterns
    // let mut filtered_tokens = Vec::new();
    // for line in inp_str.lines() {
    //     let mut tokens_for_line = bpe.encode_ordinary(line);
    //     if !line.contains("{{") && !line.contains("}}") {
    //         filtered_tokens.extend(tokens_for_line.drain(..));
    //     }
    // }
    let n_take_from_beginning = ((input_len as f32) * split).ceil() as usize;
    let n_take_from_end = (max_len as usize) - n_take_from_beginning;

    let mut concatenated_tokens = Vec::with_capacity(max_len as usize);
    concatenated_tokens.extend_from_slice(&input_token_vec[..n_take_from_beginning]);
    concatenated_tokens.extend_from_slice(&input_token_vec[input_len - n_take_from_end..]);

    bpe.decode(concatenated_tokens)
        .ok()
        .map_or("failed to decode tokens".to_string(), |s| s.to_string())
}

pub async fn chain_of_chat(
    sys_prompt_1: &str,
    usr_prompt_1: &str,
    chat_id: &str,
    gen_len_1: u16,
    usr_prompt_2: &str,
    gen_len_2: u16,
    error_tag: &str,
) -> anyhow::Result<String> {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(USER_AGENT, HeaderValue::from_static("MyClient/1.0.0"));
    let config = LocalServiceProviderConfig {
        // api_base: String::from("http://127.0.0.1:8080/v1"),
        // api_base: String::from("http://52.37.228.1:8080/v1"),
        api_base: String::from("https://api.deepinfra.com/v1/openai/chat/completions"),
        headers: headers,
        api_key: Secret::new("lY2h5Vd5wgdyICzjOyDmmmToeU3KyLgv".to_string()),
        query: HashMap::new(),
    };

    let model = "DEEP_API_KEY-must-be-set";
    let client = OpenAIClient::with_config(config);

    let mut messages = vec![
        ChatCompletionRequestSystemMessageArgs::default()
            .content(sys_prompt_1)
            .build()
            .expect("Failed to build system message")
            .into(),
        ChatCompletionRequestUserMessageArgs::default()
            .content(usr_prompt_1)
            .build()?
            .into(),
    ];
    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(gen_len_1)
        .model(model)
        .messages(messages.clone())
        .build()?;

    // dbg!("{:?}", request.clone());

    let chat = client.chat().create(request).await?;

    match chat.choices[0].message.clone().content {
        Some(res) => {
            println!("{:?}", res);
        }
        None => {
            return Err(anyhow::anyhow!(error_tag.to_string()));
        }
    }

    messages.push(
        ChatCompletionRequestUserMessageArgs::default()
            .content(usr_prompt_2)
            .build()?
            .into(),
    );

    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(gen_len_2)
        .model(model)
        .messages(messages)
        .build()?;

    let chat = client.chat().create(request).await?;

    match chat.choices[0].message.clone().content {
        Some(res) => {
            println!("{:?}", res);
            Ok(res)
        }
        None => {
            return Err(anyhow::anyhow!(error_tag.to_string()));
        }
    }
}

#[derive(Clone, Debug)]
pub struct LocalServiceProviderConfig {
    pub api_base: String,
    pub headers: HeaderMap,
    pub api_key: Secret<String>,
    pub query: HashMap<String, String>,
}

impl Config for LocalServiceProviderConfig {
    fn headers(&self) -> HeaderMap {
        self.headers.clone()
    }

    fn url(&self, path: &str) -> String {
        format!("{}{}", self.api_base, path)
    }

    fn query(&self) -> Vec<(&str, &str)> {
        self.query
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect()
    }

    fn api_base(&self) -> &str {
        &self.api_base
    }

    fn api_key(&self) -> &Secret<String> {
        &self.api_key
    }
}

pub async fn chat_inner_async(
    system_prompt: &str,
    user_input: &str,
    max_token: u16,
    model: &str,
) -> anyhow::Result<String> {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(USER_AGENT, HeaderValue::from_static("MyClient/1.0.0"));
    let config = LocalServiceProviderConfig {
        // api_base: String::from("http://127.0.0.1:8080/v1"),
        // api_base: String::from("http://10.0.0.174:8080/v1"),
        api_base: String::from("https://api.deepinfra.com/v1/openai"),
        headers: headers,
        api_key: Secret::new("lY2h5Vd5wgdyICzjOyDmmmToeU3KyLgv".to_string()),
        query: HashMap::new(),
    };

    let client = OpenAIClient::with_config(config);
    let messages = vec![
        ChatCompletionRequestSystemMessageArgs::default()
            .content(system_prompt)
            .build()
            .expect("Failed to build system message")
            .into(),
        ChatCompletionRequestUserMessageArgs::default()
            .content(user_input)
            .build()?
            .into(),
    ];
    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(max_token)
        .model(model)
        .messages(messages)
        .build()?;

    let chat = match client.chat().create(request).await {
        Ok(chat) => chat,
        Err(_e) => {
            println!("Error getting response from OpenAI: {:?}", _e);
            panic!();
        }
    };

    match chat.choices[0].message.clone().content {
        Some(res) => {
            // println!("{:?}", chat.choices[0].message.clone());
            Ok(res)
        }
        None => Err(anyhow::anyhow!("Failed to get reply from OpenAI")),
    }
}

pub fn parse_summary_from_raw_json(input: &str) -> String {
    #[derive(Deserialize, Debug)]
    struct SummaryStruct {
        impactful: Option<String>,
        alignment: Option<String>,
        patterns: Option<String>,
        synergy: Option<String>,
        significance: Option<String>,
    }

    let summary: SummaryStruct = serde_json::from_str(input).expect("Failed to parse summary JSON");

    let mut output = String::new();

    let fields = [
        &summary.impactful,
        &summary.alignment,
        &summary.patterns,
        &summary.synergy,
        &summary.significance,
    ];

    fields
        .iter()
        .filter_map(|&field| field.as_ref()) // Convert Option<&String> to Option<&str>
        .filter(|field| !field.is_empty()) // Filter out empty strings
        .fold(String::new(), |mut acc, field| {
            if !acc.is_empty() {
                acc.push_str(" ");
            }
            acc.push_str(field);
            acc
        })
}

pub async fn github_http_get(url: &str) -> anyhow::Result<Vec<u8>> {
    let token = std::env::var("GITHUB_TOKEN").expect("github_token is required");
    let mut writer = Vec::new();
    let url = Uri::try_from(url).unwrap();

    match Request::new(&url)
        .method(Method::GET)
        .header("User-Agent", "flows-network connector")
        .header("Content-Type", "application/json")
        .header("Authorization", &format!("Bearer {}", token))
        .header("CONNECTION", "close")
        .send(&mut writer)
    {
        Ok(res) => {
            if !res.status_code().is_success() {
                println!("Github http error {:?}", res.status_code());
                return Err(anyhow::anyhow!("Github http error {:?}", res.status_code()));
            }
            Ok(writer)
        }
        Err(_e) => {
            println!("Error getting response from Github: {:?}", _e);
            Err(anyhow::anyhow!(_e))
        }
    }
}

pub fn parse_issue_summary_from_json(input: &str) -> anyhow::Result<Vec<(String, String)>> {
    let parsed: serde_json::Map<String, serde_json::Value> = serde_json::from_str(input)?;

    let summaries = parsed
        .iter()
        .filter_map(|(key, value)| {
            if let Some(summary_str) = value.as_str() {
                Some((key.clone(), summary_str.to_owned()))
            } else {
                None
            }
        })
        .collect::<Vec<(String, String)>>(); // Collect into a Vec of tuples

    Ok(summaries)
}

pub async fn test_gen() -> anyhow::Result<()> {
    let text = include_str!("/Users/jichen/Projects/local-llm-tester/src/raw_commit.txt");
    let raw_len = text.len();
    let stripped_texts = text.chars().take(24_000).collect::<String>();
    // let stripped_texts = String::from_utf8(response).ok()?.chars().take(24_000).collect::<String>();
    let user_name = "Akihiro Suda".to_string();

    let tag_line = "commit".to_string();

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
        "chained-prompt-commit",
    )
    .await?;

    println!("len: {} Summary: {:?}", raw_len, summary.clone());

    use std::fs::File;
    use std::io::Write;
    let mut file = File::create("commits.txt").expect("create failed");

    // Iterate over the map and write each entry to the file
    // `value.0` is the URL and `value.1` is the summary in this context
    file.write_all(summary.as_bytes()).expect("write failed");
    Ok(())
}
