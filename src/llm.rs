// use http_req::{ request::Method, request::Request, response, uri::Uri };
// use log;
use reqwest::header::{ HeaderMap, HeaderValue, CONTENT_TYPE, USER_AGENT, AUTHORIZATION };
use reqwest::{ header, Client };
use secrecy::{
    // ExposeSecret,
    Secret,
};
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
use std::env;
use serde::{ Serialize, Deserialize };

pub async fn chain_of_chat(
    sys_prompt_1: &str,
    usr_prompt_1: &str,
    chat_id: &str,
    gen_len_1: u16,
    usr_prompt_2: &str,
    gen_len_2: u16,
    error_tag: &str
) -> anyhow::Result<String> {
    let token = env::var("LLM_API_KEY").unwrap_or(String::from("LLM_API_KEY-must-be-set"));

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(USER_AGENT, HeaderValue::from_static("MyClient/1.0.0"));
    let config = LocalServiceProviderConfig {
        // api_base: String::from("http://127.0.0.1:8080/v1"),
        api_base: String::from("http://52.37.228.1:8080/v1"),
        headers: headers,
        api_key: Secret::new(token),
        query: HashMap::new(),
    };

    let model = "mistralai/Mistral-7B-Instruct-v0.1";
    let client = OpenAIClient::with_config(config);

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
        .model(model)
        .messages(messages.clone())
        .build()?;

    // dbg!("{:?}", request.clone());

    let chat = client.chat().create(request).await?;

    match chat.choices[0].message.clone().content {
        Some(res) => {
            println!("step 1 Points: {:?}", res);
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
        .model(model)
        .messages(messages)
        .build()?;

    let chat = client.chat().create(request).await?;

    match chat.choices[0].message.clone().content {
        Some(res) => {
            println!("step 2 Raw: {:?}", res);
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
    model: &str
) -> anyhow::Result<String> {
    let token = env::var("LLM_API_KEY").unwrap_or(String::from("LLM_API_KEY-must-be-set"));
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(USER_AGENT, HeaderValue::from_static("MyClient/1.0.0"));
    let config = LocalServiceProviderConfig {
        // api_base: String::from("http://127.0.0.1:8080/v1"),
        api_base: String::from("http://52.37.228.1:8080/v1"),
        headers: headers,
        api_key: Secret::new(token),
        query: HashMap::new(),
    };

    let client = OpenAIClient::with_config(config);
    let messages = vec![
        ChatCompletionRequestSystemMessageArgs::default()
            .content(system_prompt)
            .build()
            .expect("Failed to build system message")
            .into(),
        ChatCompletionRequestUserMessageArgs::default().content(user_input).build()?.into()
    ];
    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(max_token)
        .model(model)
        .messages(messages)
        .build()?;

    match client.chat().create(request).await {
        Ok(chat) =>
            match chat.choices[0].message.clone().content {
                Some(res) => {
                    // println!("{:?}", chat.choices[0].message.clone());
                    Ok(res)
                }
                None => Err(anyhow::anyhow!("Failed to get reply from OpenAI")),
            }
        Err(_e) => {
            println!("Error getting response from hosted LLM: {:?}", _e);
            Err(anyhow::anyhow!(_e))
        }
    }
}

pub async fn completion_inner_async(user_input: &str) -> anyhow::Result<String> {
    let llm_endpoint = "https://api-inference.huggingface.co/models/jaykchen/tiny".to_string();
    let llm_api_key = env::var("LLM_API_KEY").expect("LLM_API_KEY-must-be-set");

    let client = Client::new();

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", llm_api_key)).unwrap()
    );

    let body = serde_json::json!({
        "inputs": user_input,
    });

    use anyhow::Context;

    let response = client
        .post(llm_endpoint)
        .headers(headers)
        .json(&body)
        .send().await
        .context("Failed to send request to API")?; // Adds context to the error

    let status_code = response.status();

    if status_code.is_success() {
        let response_body = response.text().await.context("Failed to read response body")?;

        let completion_response: Vec<Choice> = serde_json
            ::from_str(&response_body)
            .context("Failed to parse response from API")?;

        if let Some(choice) = completion_response.get(0) {
            Ok(choice.generated_text.clone())
        } else {
            Err(anyhow::anyhow!("No completion choices found in the response"))
        }
    } else {
        Err(anyhow::anyhow!("Failed to get a successful response from the API"))
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Choice {
    pub generated_text: String,
}
