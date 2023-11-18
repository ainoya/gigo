use std::error::Error;

use async_trait::async_trait;
use reqwest::Client;
use serde_json::{json, Value};

use super::apiclient::ApiClient;

pub struct OpenAIApiClient {
    api_key: String,
    client: reqwest::Client,
    endpoint: String,
}

impl OpenAIApiClient {
    pub fn new(api_key: String, endpoint: String, client: Client) -> Self {
        OpenAIApiClient {
            api_key,
            client,
            endpoint,
        }
    }
}

#[async_trait]
impl ApiClient for OpenAIApiClient {
    async fn post(&self, instruction: &str, message: &str) -> Result<String, Box<dyn Error>> {
        let auth_header = format!("Bearer {}", self.api_key);

        let chat_endpoint = format!("{}/v1/chat/completions", self.endpoint);

        let response = self
            .client
            .post(chat_endpoint)
            .header("Authorization", auth_header)
            .json(&json!(
              {
                "model": "gpt-3.5-turbo",
                "messages": [
                    {
                      "role": "system",
                      "content": instruction
                    },
                    {
                        "role": "user",
                        "content": message,
                    }
                ]
              }
            ))
            .send()
            .await?
            .text()
            .await?;

        // get output
        let json: Value = serde_json::from_str(&response)?;
        // let output = json["choices"][0]["message"]["content"].as_str().unwrap();

        // print response
        // println!("Response: {}", json);

        let msg_opt = json["choices"][0]["message"]["content"].as_str().unwrap();

        match msg_opt {
            "None" => Ok(String::from("")),
            _ => Ok(String::from(msg_opt)),
        }
    }
}
