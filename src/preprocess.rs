// use http_req::{ request::Method, request::Request, response, uri::Uri };
// use log;
// use reqwest::header::{ HeaderMap, AUTHORIZATION };
// use reqwest::{ header, Client };
use crate::llm::chat_inner_async;

pub fn chunk_json_vec(json_vec: Vec<Vec<String>>, chunk_size: usize) -> Vec<String> {
    let mut out = Vec::new();

    for ve in json_vec {
        let mut current_chunk = String::new();
        for line in ve {
            if current_chunk.is_empty() {
                current_chunk.push_str(&line);
            } else {
                if current_chunk.len() + line.len() + 1 > chunk_size {
                    out.push(current_chunk);
                    current_chunk = line;
                } else {
                    current_chunk.push('\n');
                    current_chunk.push_str(&line);
                }
            }
        }
        if !current_chunk.is_empty() {
            out.push(current_chunk);
        }
    }

    out
}

pub async fn summarize_long_chunks(input: &str) -> String {
    let sys_prompt_1 = format!("You're a technical edtior bot.");

    let usr_prompt_1 = format!(
        "To prepare for downstream question & answer task, you need to proprocess the source material, there are long chunks of text that are tool long to use as context, you need to extract the essence of such chunks, now please summarize this chunk: `{input}` into one concise paragraph, please stay truthful to the source material and handle the task in a factual manner."
    );
    let model = "mistralai/Mistral-7B-Instruct-v0.1";

    chat_inner_async(&sys_prompt_1, &usr_prompt_1, 128, model)
        .await
        .ok()
        .unwrap_or(String::new())
}
