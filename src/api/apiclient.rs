use std::error::Error;

use async_trait::async_trait;

#[async_trait]
pub trait ApiClient {
    async fn post(&self, instruction: &str, message: &str) -> Result<String, Box<dyn Error>>;
}
