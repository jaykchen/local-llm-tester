// use http_req::{ request::Method, request::Request, response, uri::Uri };
// use log;
// use reqwest::header::{ HeaderMap, AUTHORIZATION };
// use reqwest::{ header, Client };
use crate::llm::chat_inner_async;

pub fn chunk_for_embed(json_vec: Vec<Vec<String>>, chunk_size: usize) -> Vec<String> {
    let mut out = Vec::new();
    let mut i = 0;

    while i < json_vec.len() {
        let mut current_chunk = String::new();
        let mut current_chunk_size = 0;

        while i < json_vec.len() && json_vec[i].len() == 1 {
            let line = &json_vec[i][0];
            let line_len = line.len() + (if current_chunk.is_empty() { 0 } else { 1 });

            if current_chunk_size + line_len > chunk_size && !current_chunk.is_empty() {
                out.push(current_chunk);
                current_chunk = String::new();
                current_chunk_size = 0;
            }

            if !current_chunk.is_empty() {
                current_chunk.push('\n');
                current_chunk_size += 1;
            }
            current_chunk.push_str(line);
            current_chunk_size += line.len();

            i += 1; // Move to the next Vec
        }

        if i < json_vec.len() && json_vec[i].len() > 1 {
            for line in &json_vec[i] {
                let line_len = line.len() + (if current_chunk.is_empty() { 0 } else { 1 });
                if current_chunk_size + line_len > chunk_size {
                    out.push(current_chunk);
                    current_chunk = String::new();
                    current_chunk_size = 0;
                }

                if !current_chunk.is_empty() {
                    current_chunk.push('\n');
                    current_chunk_size += 1;
                }
                current_chunk.push_str(line);
                current_chunk_size += line.len();
            }
            i += 1; // Move past the paragraph
        }

        if !current_chunk.is_empty() {
            out.push(current_chunk);
        }
    }

    out
}

pub fn chunk_for_summarize(json_vec: Vec<Vec<String>>, ingest_window_size: usize) -> Vec<String> {
    let mut out = Vec::new();
    let mut current_chunk = String::new();

    for ve in json_vec.iter() {
        let section = ve.join("\n");
        let section_size = section.len();

        if !current_chunk.is_empty() && current_chunk.len() + 1 + section_size > ingest_window_size {
            out.push(current_chunk);
            current_chunk = section;
        } else {
            if !current_chunk.is_empty() {
                current_chunk.push('\n'); // Add a newline before adding the section.
            }
            current_chunk.push_str(&section);
        }
    }

    if !current_chunk.is_empty() {
        out.push(current_chunk);
    }

    out
}

pub async fn summarize_long_chunks(input: &str) -> String {
    let sys_prompt_1 = format!("You're a technical edtior bot.");

    let usr_prompt_1 = format!(
        "To prepare for downstream question & answer task, you need to proprocess the source material, there are long chunks of text that are tool long to use as context, you need to extract the essence of such chunks, now please summarize this chunk: `{input}` into one concise paragraph, please stay truthful to the source material and handle the task in a factual manner."
    );
    let model = "mistralai/Mistral-7B-Instruct-v0.1";

    chat_inner_async(&sys_prompt_1, &usr_prompt_1, 128, model).await.ok().unwrap_or(String::new())
}
