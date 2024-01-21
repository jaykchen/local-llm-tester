use clap::{ App, Arg };
use dotenv::dotenv;
use local_llm_tester::embed::*;
use local_llm_tester::preprocess::*;
use local_llm_tester::rag_logic::*;
use local_llm_tester::utils::*;
use qdrant_client::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    // let OPENAI_API_KEY = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY is not set");

    /*     let matches = App::new("local-llm-tester")
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
    .get_matches(); */

    let client = QdrantClient::from_url("http://10.0.0.174:6334").build()?;

    // let collections_list = client.list_collections().await?;
    // dbg!(collections_list);

    let collection_name = "k8s";

    // let _ = create_collection(embedding_dims, &client, collection_name).await;

    // client.delete_collection(collection_name).await?;

    // let collection_info = client.collection_info(collection_name).await?;
    // dbg!(collection_info);

    let text_json = include_str!("segmented_text.json");

    let text_json: Vec<Vec<String>> = serde_json::from_str(text_json).unwrap();

    // let _ = populate_collection(text_json, &client, collection_name).await;

    // return Ok(());
    let question = "What is the meaning of kubernetes?";
    let question = "What are the charcteristic of kubernetes?";
    let hypo_answer = create_hypothetical_answer(question).await;
    println!("hypo_answer: {}", hypo_answer);

    let original_texts_pulled = search_collection(&question, &client, collection_name).await?;

    println!("original_texts_pulled: {:?}", original_texts_pulled);
    let hypo_texts_pulled = search_collection(&hypo_answer, &client, collection_name).await?;
println!("hypo_texts_pulled: {:?}", hypo_texts_pulled);

    let context_text = pooling_candidate_texts(original_texts_pulled, hypo_texts_pulled, 20_000);

    let system_prompt = format!("You're a question & answer bot on a specified source material.");

    let user_input = format!(
        "Use these source material as context: `{context_text}`, please provide a concise answer."
    );
    let model = "mistralai/Mistral-7B-Instruct-v0.1";

    let answer = chat_inner_async(&system_prompt, &user_input, 512, model).await?;

    println!("answer: {}", answer);

    Ok(())
}
