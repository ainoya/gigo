mod chat;
use chat::call_chat_api;
use gigo::api::openaiapiclient::OpenAIApiClient;
use std::env;
use std::io::{self, Read}; // Import the env module

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut buffer = String::new();

    match io::stdin().read_to_string(&mut buffer) {
        Ok(_) => (),
        Err(e) => return Err(e),
    }

    // Get the OpenAI API key from the environment variable
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");

    let client = OpenAIApiClient::new(
        api_key,
        "https://api.openai.com".to_string(),
        reqwest::Client::new(),
    );

    let instructions = "The following is a conversation with an AI assistant. The assistant is helpful, creative, clever, and very friendly.";

    let res = call_chat_api(&client, &instructions, &buffer).await;

    match res {
        Ok(response) => println!("{}", response),
        Err(e) => println!("Error: {}", e),
    }

    Ok(())
}
