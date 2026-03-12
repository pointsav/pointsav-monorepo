use reqwest::{Client, Error};
use serde_json::Value;

pub struct GraphBridge {
    client: Client,
    access_token: String,
}

impl GraphBridge {
    pub fn new(token: String) -> Self {
        GraphBridge {
            client: Client::new(),
            access_token: token,
        }
    }

    pub async fn fetch_inbox(&self, user_id: &str) -> Result<Value, Error> {
        let url = format!("https://graph.microsoft.com/v1.0/users/{}/messages", user_id);
        
        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.access_token))
            .send()
            .await?;

        let payload: Value = response.json().await?;
        Ok(payload)
    }
}
