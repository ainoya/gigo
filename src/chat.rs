use gigo::api::apiclient::ApiClient;
use std::error::Error;

pub async fn call_chat_api(
    client: &dyn ApiClient,
    instruction: &str,
    message: &str,
) -> Result<String, Box<dyn Error>> {
    client.post(instruction, message).await
}

// mock ApiClient
#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;

    struct MockApiClient {}

    #[async_trait]
    impl ApiClient for MockApiClient {
        async fn post(&self, instruction: &str, message: &str) -> Result<String, Box<dyn Error>> {
            Ok(format!("{} {}", instruction, message))
        }
    }

    #[tokio::test]
    async fn test_call_chat_api() {
        let client = MockApiClient {};
        let instruction = "instruction";
        let message = "message";

        let res = call_chat_api(&client, &instruction, &message).await;

        assert_eq!(res.unwrap(), "instruction message");
    }

    // handle error
    #[tokio::test]
    async fn test_call_chat_api_error() {
        struct MockApiClient {}

        #[async_trait]
        impl ApiClient for MockApiClient {
            async fn post(
                &self,
                _instruction: &str,
                _message: &str,
            ) -> Result<String, Box<dyn Error>> {
                Err("error".into())
            }
        }

        let client = MockApiClient {};
        let instruction = "instruction";
        let message = "message";

        let res = call_chat_api(&client, &instruction, &message).await;

        assert_eq!(res.unwrap_err().to_string(), "error");
    }
}
