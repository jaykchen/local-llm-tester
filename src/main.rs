pub mod data_analyzers;
pub mod github_data_fetchers;
pub mod utils;
use utils::*;
use data_analyzers::*;
use dotenv::dotenv;
use github_data_fetchers::*;
use serde_json::Value;
use std::collections::HashMap;
use std::env;

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
    let n_days = 7u16;

    let text_json = include_str!("segmented_text.json");

    let text_json: Vec<Vec<String>> = serde_json::from_str(text_json).unwrap();
    let mut id = 0;
    for line in text_json.into_iter().flatten() {
        match create_embed(&line).await {
            Ok(emb) => {
                println!("Line: {}\n emb: {:?}", line, emb[0..10].to_vec());

                id += 1;
            }
            Err(e) => {
                log::error!("OpenAI returned an error: {}", e);
            }
        }
        break;
    }

    Ok(())
}
