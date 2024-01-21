// use http_req::{ request::Method, request::Request, response, uri::Uri };
// use log;
// use reqwest::header::{ HeaderMap, AUTHORIZATION };
// use reqwest::{ header, Client };
// use secrecy::{ ExposeSecret, Secret };
// use serde::{ Deserialize, Serialize };
use crate::embed::*;
use crate::llm::*;
use crate::preprocess::*;
use qdrant_client::prelude::*;
use qdrant_client::qdrant::{
    // Filter,
    // PointId,
    point_id::PointIdOptions,
    vectors_config,
    // Condition,
    CreateCollection,
    SearchPoints,
    // Vector,
    VectorParams,
    VectorsConfig,
};
use serde_json::json;
use std::collections::HashMap;

pub async fn create_hypothetical_answer(question: &str) -> String {
    let sys_prompt_1 = format!(
        "You're an assistant bot with expertise in all domains of human knowledge."
    );

    let usr_prompt_1 = format!(
        "You're preparing to answer questions about a specific source material, before ingesting the source material, you need to answer the question based on the knowledge you're trained on, here it is: `{question}`, please provide a concise answer in one paragraph, stay truthful and factual."
    );
    let model = "mistralai/Mistral-7B-Instruct-v0.1";

    chat_inner_async(&sys_prompt_1, &usr_prompt_1, 128, model).await.ok().unwrap_or(String::new())
}

pub fn pooling_candidate_texts(
    original_texts_pulled: Vec<(u64, f32, String)>,
    hypo_texts_pulled: Vec<(u64, f32, String)>,
    target_length: usize
) -> String {
    let mut map = original_texts_pulled
        .into_iter()
        .map(|(id, _, text)| (id, text))
        .collect::<HashMap<u64, String>>();

    for (id, _, text) in hypo_texts_pulled.into_iter() {
        map.entry(id)
            .and_modify(|e| {
                e.push_str("\n");
                e.push_str(&text);
            })
            .or_insert(text);
    }

    map.into_iter()
        .map(|(_, text)| text)
        .collect::<Vec<String>>()
        .join("\n")
        .chars()
        .take(target_length)
        .collect::<String>()
}

pub async fn populate_collection(
    text_json: Vec<Vec<String>>,
    client: &QdrantClient,
    collection_name: &str
) {
    let mut id = 0;
    for line in chunk_json_vec(text_json, 2000) {
        match create_embed_req(&line).await {
            Ok(emb) => {
                println!("Block embed processed: {}", line.chars().take(20).collect::<String>());

                let payload: Payload = json!({
            "text": line}).try_into().unwrap();

                let point = PointStruct {
                    id: Some(id.into()),
                    payload: payload.into(),
                    vectors: Some(emb.into()),
                    ..Default::default()
                };
                let _ = client.upsert_points_blocking(
                    collection_name,
                    None,
                    vec![point],
                    None
                ).await;

                id += 1;
            }
            Err(e) => {
                println!("LLM returned an error: {}", e);
            }
        }
    }
}
pub async fn create_collection(
    embedding_dims: u64,
    client: &QdrantClient,
    collection_name: &str
) -> anyhow::Result<()> {
    match
        client.create_collection(
            &(CreateCollection {
                collection_name: collection_name.into(),
                vectors_config: Some(VectorsConfig {
                    config: Some(
                        vectors_config::Config::Params(VectorParams {
                            size: embedding_dims,
                            distance: Distance::Cosine.into(),
                            ..Default::default()
                        })
                    ),
                }),
                ..Default::default()
            })
        ).await
    {
        Ok(_) => Ok(()),
        Err(_e) => Err(anyhow::anyhow!("Collection already exists")),
    }
}

pub async fn search_collection(
    question: &str,
    client: &QdrantClient,
    collection_name: &str
) -> anyhow::Result<Vec<(u64, f32, String)>> {
    let query_emb = create_embed_req(question).await?;
    let search_result = client.search_points(
        &(SearchPoints {
            collection_name: collection_name.into(),
            vector: query_emb,
            filter: None,
            // filter: Some(Filter::all([Condition::matches("bar", 12)])),
            limit: 5,
            ..Default::default()
        })
    ).await?;

    let mut results = Vec::<(u64, f32, String)>::new();
    for res in search_result.result.into_iter() {
        let id = if let Some(PointIdOptions::Num(num)) = res.id.unwrap().point_id_options {
            num
        } else {
            // You can handle the Uuid case here if needed, or continue to the next result
            continue;
        };
        let score = res.score;
        let payload_str = match res.payload.get("text") {
            Some(payload) => payload.as_str().unwrap().to_string(),

            None => {
                continue;
            }
        };

        results.push((id, score, payload_str));
    }

    Ok(results)
}
