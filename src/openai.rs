use reqwest::Client;
use serde_json::json;
use serde_json::Value;
use std::error::Error;

pub async fn call_openai_api(input: &str, api_key: &str) -> Result<String, Box<dyn Error>> {
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()?;

    let auth_header = format!("Bearer {}", api_key);

    // print input
    println!("Input: {}", input);

    // call chat completion endpoint
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", auth_header)
        .json(&json!({
            "model": "gpt-3.5-turbo",
            "messages": [
                {
                    "role": "user",
                    "content": input,
                }
            ]
        }))
        .send()
        .await?
        .text()
        .await?;

    // get output
    let json: Value = serde_json::from_str(&response)?;
    // let output = json["choices"][0]["message"]["content"].as_str().unwrap();

    // print response
    println!("Response: {}", json);

    let output: &str = json["choices"][0]["message"]["content"].as_str().unwrap();

    Ok(output.to_string())
}
