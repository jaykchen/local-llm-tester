pub mod data_analyzers;
pub mod embed;
pub mod github_data_fetchers;
pub mod utils;
use anyhow::Result;
use data_analyzers::*;
use dotenv::dotenv;
use embed::*;
use github_data_fetchers::*;
use octocrab::models::events::payload;
use qdrant_client::prelude::*;
use qdrant_client::qdrant::vectors_config::Config;
use qdrant_client::qdrant::{
    Condition,
    CreateCollection,
    Filter,
    PointId,
    SearchPoints,
    Vector,
    VectorParams,
    VectorsConfig,
};
use serde_json::{ json, Value };
use std::collections::HashMap;
use std::env;
use utils::*;

use clap::{ App, Arg };

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let OPENAI_API_KEY = String::from("");
    // let OPENAI_API_KEY = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY is not set");

    let matches = App::new("local-llm-tester")
        .version("1.0")
        .arg(
            Arg::with_name("owner")
                .long("owner")
                .help("Specifies the owner of the repository")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("repo")
                .long("repo")
                .help("Specifies the repository name")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("username")
                .long("username")
                .help("Specifies the GitHub username")
                .takes_value(true)
        )
        .get_matches();

    let client = QdrantClient::from_url("http://10.0.0.174:6334").build()?;

    // let collections_list = client.list_collections().await?;
    // dbg!(collections_list);
    // collections_list = ListCollectionsResponse {
    //     collections: [
    //         CollectionDescription {
    //             name: "test",
    //         },
    //     ],
    //     time: 1.78e-6,
    // }

    let collection_name = "k8s";
    client.delete_collection(collection_name).await?;

    // client.create_collection(
    //     &(CreateCollection {
    //         collection_name: collection_name.into(),
    //         vectors_config: Some(VectorsConfig {
    //             config: Some(
    //                 Config::Params(VectorParams {
    //                     size: 1024,
    //                     distance: Distance::Cosine.into(),
    //                     ..Default::default()
    //                 })
    //             ),
    //         }),
    //         ..Default::default()
    //     })
    // ).await?;

    // let collection_info = client.collection_info(collection_name).await?;
    // dbg!(collection_info);

    // let points = vec![PointStruct::new(0, vec![12.; 10], payload)];

    let text_json = include_str!("segmented_text.json");

    let text_json: Vec<Vec<String>> = serde_json::from_str(text_json).unwrap();
    let mut id = 0;
    for line in text_json.into_iter().flatten() {
        let mut id = 0; // Assuming `id` is initialized here and is mutable

        match create_embed_req(&line).await {
            Ok(emb) => {
                println!("Line: {}\n emb: {:?}", line, emb.to_vec());

                let payload: Payload = json!({"text": line})
                    .try_into()
                    .expect("Failed to convert into Payload");

                let point = PointStruct {
                    id: Some(id.into()), // Make sure `id` is mutable and defined outside of this closure
                    payload: payload.into(),
                    vectors: Some(emb.into()),
                    ..Default::default()
                };

                id += 1; // Increment `id` for the next iteration
                client.upsert_points(collection_name, None, vec![point], None).await?;
            }
            Err(e) => {
                println!("Error processing line: {}", e);
            }
        }

        break;
    }

    /*     let search_result = client.search_points(
        &(SearchPoints {
            collection_name: collection_name.into(),
            vector: vec![11.; 10],
            filter: Some(Filter::all([Condition::matches("bar", 12)])),
            limit: 10,
            with_payload: Some(true.into()),
            ..Default::default()
        })
    ).await?; */

    // search_result = SearchResponse {
    //     result: [
    //         ScoredPoint {
    //             id: Some(
    //                 PointId {
    //                     point_id_options: Some(
    //                         Num(
    //                             0,
    //                         ),
    //                     ),
    //                 },
    //             ),
    //             payload: {
    //                 "bar": Value {
    //                     kind: Some(
    //                         IntegerValue(
    //                     12,
    //                     ),
    //                     ),
    //                 },
    //                 "foo": Value {
    //                     kind: Some(
    //                         StringValue(
    //                     "Bar",
    //                     ),
    //                     ),
    //                 },
    //             },
    //             score: 1.0000001,
    //             version: 0,
    //             vectors: None,
    //         },
    //     ],
    //     time: 9.5394e-5,
    // }

    // let found_point = search_result.result.into_iter().next().unwrap();
    // let mut payload = found_point.payload;
    // let baz_payload = payload.remove("baz").unwrap().into_json();
    // println!("baz: {}", baz_payload);
    Ok(())
}
