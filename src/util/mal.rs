use reqwest;
use serde_json::json;
use std::collections::HashMap;

pub struct MALConfig {
    client_id: String,
    client_secret: String,
    access_token: String,
    refresh_token: String,
}

impl MALConfig {
    pub fn new(
        client_id: impl ToString,
        client_secret: impl ToString,
        access_token: impl ToString,
        refresh_token: impl ToString,
    ) -> MALConfig {
        MALConfig {
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
            access_token: access_token.to_string(),
            refresh_token: refresh_token.to_string(),
        }
    }

    fn headers(&self) -> HashMap<String, String> {
        let mut headers = HashMap::new();
        headers.insert(
            String::from("Authorization"),
            format!("Bearer {}", self.access_token),
        );

        headers
    }

    pub async fn regen_token(&mut self) -> String {
        let client = reqwest::Client::new();
        let response = client
            .post("https://myanimelist.net/v1/oauth2/token")
            .json(&json!({"a": "b"}))
            .send()
            .await;

        response.unwrap().json().await.unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn works() {
        let _ = MALConfig::new(
            "client_id",
            "client_secret",
            "access_token",
            "refresh_token",
        );
    }
}
