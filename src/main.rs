mod openai;
use openai::call_openai_api;
use std::io::{self, Read};
use std::env; // Import the env module

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    // Get the OpenAI API key from the environment variable
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");

    match call_openai_api(&buffer, &api_key).await {
        Ok(response) => println!("{}", response),
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}
