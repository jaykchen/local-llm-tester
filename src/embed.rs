use http_req::{request::Method, request::Request, response, uri::Uri};
use log;
use reqwest::header::{HeaderMap, AUTHORIZATION};
use reqwest::{header, Client};
use secrecy::{ExposeSecret, Secret};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::collections::HashSet;

use async_openai::{
    config::Config,
    types::{
        // ChatCompletionFunctionsArgs, ChatCompletionRequestMessage,
        ChatCompletionRequestSystemMessageArgs,
        ChatCompletionRequestUserMessageArgs,
        // ChatCompletionTool, ChatCompletionToolArgs, ChatCompletionToolType,
        CreateChatCompletionRequestArgs,
        CreateEmbeddingRequest,
        // FinishReason,
        CreateEmbeddingRequestArgs,
        CreateEmbeddingResponse,
        Embedding,
        EmbeddingUsage,
    },
    Client as OpenAIClient, Models,
};
use std::env;

pub async fn create_embed(input: &str) -> anyhow::Result<Vec<f32>> {
    use reqwest::header::{HeaderValue, CONTENT_TYPE, USER_AGENT};
    let token = env::var("DEEP_API_KEY").unwrap_or(String::from("DEEP_API_KEY-must-be-set"));
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(USER_AGENT, HeaderValue::from_static("MyClient/1.0.0"));
    let config = LocalServiceProviderConfig {
        api_base: String::from("http://52.37.228.1:3000/"),
        headers: headers,
        api_key: Secret::new(token),
        query: HashMap::new(),
    };

    let client = OpenAIClient::with_config(config);

    // Create a request as before
    let request = CreateEmbeddingRequestArgs::default()
        .model("BAAI/bge-large-en-v1.5")
        .input(input)
        .build()
        .unwrap();
    dbg!("request", &request);
    match client.embeddings().create(request).await {
        Ok(response) => Ok(response.data.get(0).unwrap().embedding.clone()),
        Err(_e) => {
            println!("Error getting response from hosted LLM: {:?}", _e);
            Err(anyhow::Error::msg("{_e}".to_string()))
        }
    }
}

pub async fn create_embed_req(input: &str) -> anyhow::Result<Vec<f32>> {
    let mut writer = Vec::new();

    let uri = Uri::try_from("http://52.37.228.1:3000/embeddings").unwrap();
    let json_body = serde_json::json!(
        {
            "model": "BAAI/bge-large-en-v1.5",
            "input": input
        }
    );
    let body = serde_json::to_vec(&json_body)?;
    match http_req::request::Request::new(&uri)
        .method(Method::POST)
        .header("Content-Type", "application/json")
        .header("Content-Length", &body.len())
        .body(&body)
        .send(&mut writer)
    {
        Ok(res) => match res.status_code().is_success() {
            true => {
                let obj = serde_json::from_slice::<CreateEmbeddingResponse>(&writer).unwrap();
                Ok(obj.data.get(0).unwrap().embedding.clone())
            }
            false => Err(anyhow::anyhow!(
                "Error getting response from hosted LLM: {:?}",
                res.status_code()
            )),
        },
        Err(e) => Err(anyhow::anyhow!(
            "Error getting response from hosted LLM: {:?}",
            e
        )),
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
