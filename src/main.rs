pub mod data_analyzers;
pub mod github_data_fetchers;
pub mod utils;
use data_analyzers::*;
use dotenv::dotenv;
use github_data_fetchers::*;
use serde_json::Value;
use std::collections::HashMap;
use std::env;

use clap::{ App, Arg };

#[tokio::main]
async fn main() {
    dotenv().ok();
    let OPENAI_API_KEY = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY is not set");

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

    // let contributors_set;

    let mut _profile_data = String::new();
    if let (Some(owner), Some(repo)) = (matches.value_of("owner"), matches.value_of("repo")) {
        let username = matches.value_of("username").map(String::from);
        // let token = matches.value_of("token").map(String::from);

        // match is_valid_owner_repo(owner, repo).await {
        //     Err(_e) => {
        //         println!(
        //             "You've entered invalid owner/repo, or the target is private. Please try again."
        //         );
        //     }
        //     Ok((_, _, inner_set)) => {
        //         contributors_set = inner_set;
        //     }
        // }
        let mut commits_map = std::collections::HashMap::<String, (String, String)>::new();
        match get_commits_in_range_search(owner, repo, username.clone(), n_days).await {
            Some((count, commits_vec)) => {
                let _ = process_commits(commits_vec, &mut commits_map).await;
            }
            None => println!("failed to get commits"),
        }
        for (key, value) in commits_map {
            println!("user: {}  url:{} summary: {}", key, value.0, value.1);
        }
    }
}
