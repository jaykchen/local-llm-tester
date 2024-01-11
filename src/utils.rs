use reqwest::{ Client, header };

use http_req::{ request::Method, request::Request, response, uri::Uri };
use log;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashSet;

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

    let drain_end = if n_keep_till_end <= body_len { body_len - n_keep_till_end } else { 0 };

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

/* pub async fn chain_of_chat(
    sys_prompt_1: &str,
    usr_prompt_1: &str,
    chat_id: &str,
    gen_len_1: u16,
    usr_prompt_2: &str,
    gen_len_2: u16,
    error_tag: &str
) -> anyhow::Result<String> {
    let client = Client::new();

    let mut messages = vec![
        ChatCompletionRequestSystemMessageArgs::default()
            .content(sys_prompt_1)
            .build()
            .expect("Failed to build system message")
            .into(),
        ChatCompletionRequestUserMessageArgs::default().content(usr_prompt_1).build()?.into()
    ];
    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(gen_len_1)
        .model("gpt-3.5-turbo-1106")
        .messages(messages.clone())
        .build()?;

    let chat = client.chat().create(request).await?;

    match chat.choices[0].message.clone().content {
        Some(res) => {
            // println!("{:?}", res);
        }
        None => {
            return Err(anyhow::anyhow!(error_tag.to_string()));
        }
    }

    messages.push(
        ChatCompletionRequestUserMessageArgs::default().content(usr_prompt_2).build()?.into()
    );

    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(gen_len_2)
        .model("gpt-3.5-turbo-1106")
        .messages(messages)
        .build()?;

    let chat = client.chat().create(request).await?;

    match chat.choices[0].message.clone().content {
        Some(res) => {
            // println!("{:?}", res);
            Ok(res)
        }
        None => {
            return Err(anyhow::anyhow!(error_tag.to_string()));
        }
    }
} */

pub async fn chat_inner(
    system_prompt: &str,
    user_input: &str,
    max_token: u16
) -> anyhow::Result<String> {
    use serde::{ Serialize };

    #[derive(Serialize)]
    struct Payload {
        prompt: String,
        n_predict: u16,
    }

    #[derive(Deserialize)]
    struct ChatResponse {
        content: String,
    }
    let client = Client::new();

    let full_prompt = format!("{}{}", system_prompt, user_input);

    let request_payload = Payload {
        prompt: full_prompt,
        n_predict: max_token,
    };

    let response = client
        .post("http://127.0.0.1:8080/completion")
        .json(&request_payload)
        .header(header::CONTENT_TYPE, "application/json")
        .send().await?;

    if response.status().is_success() {
        let chat_response = response.json::<ChatResponse>().await?;
        println!("{:?}", chat_response.content.clone());
        Ok(chat_response.content)
    } else {
        Err(anyhow::anyhow!("Request failed with status: {}", response.status()))
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

    match
        Request::new(&url)
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
