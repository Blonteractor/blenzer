pub mod anime;
pub mod manga;
pub mod prelude;

use dotenv::dotenv;
use reqwest::{
    self,
    header::{HeaderMap, HeaderName, HeaderValue},
    Client, Response,
};
use std::collections::HashMap;
use std::env;

pub struct MALConfig {
    client_id: String,
    client_secret: String,
    access_token: String,
    refresh_token: String,
    client: Client,
}

impl MALConfig {
    pub fn new(
        client_id: impl ToString,
        client_secret: impl ToString,
        access_token: impl ToString,
        refresh_token: impl ToString,
    ) -> Self {
        Self {
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
            access_token: access_token.to_string(),
            refresh_token: refresh_token.to_string(),
            client: reqwest::Client::new(),
        }
    }

    pub fn from_env() -> Self {
        dotenv().ok();

        MALConfig::new(
            env::var("MAL_CLIENT_ID").unwrap(),
            env::var("MAL_CLIENT_SECRET").unwrap(),
            env::var("MAL_ACCESS_TOKEN").unwrap(),
            env::var("MAL_REFRESH_TOKEN").unwrap(),
        )
    }

    fn headers(&self) -> HeaderMap {
        let mut map = HeaderMap::new();
        let v = format!("Bearer {}", self.access_token);
        map.insert(
            HeaderName::from_static("authorization"),
            HeaderValue::from_str(&v).unwrap(),
        );
        map
    }

    pub async fn regen_token(&mut self) -> Result<&str, reqwest::Error> {
        let response = self
            .client
            .post("https://myanimelist.net/v1/oauth2/token")
            .query(&[
                ("client_id", self.client_id.as_str()),
                ("client_secret", self.client_secret.as_str()),
                ("grant_type", "refresh_token"),
                ("refresh_token", self.refresh_token.as_str()),
            ])
            .send()
            .await;

        let response_data = response?.json::<HashMap<String, String>>().await.unwrap();

        self.access_token = response_data
            .get("access_token")
            .unwrap_or(&self.access_token)
            .to_string();

        self.refresh_token = response_data
            .get("refresh_token")
            .unwrap_or(&self.refresh_token)
            .to_string();

        Ok(self.access_token.as_str())
    }

    pub async fn get(
        &self,
        url: &str,
        params: HashMap<&str, &str>,
    ) -> Result<Response, reqwest::Error> {
        self.client
            .get(url)
            .query(&params)
            .headers(self.headers())
            .send()
            .await
    }

    pub async fn post(
        &self,
        url: &str,
        data: HashMap<&str, &str>,
    ) -> Result<Response, reqwest::Error> {
        self.client
            .post(url)
            .query(&data)
            .headers(self.headers())
            .send()
            .await
    }
}

#[cfg(test)]
mod test {
    use dotenv::dotenv;
    use reqwest::StatusCode;

    use super::*;

    #[tokio::test]
    async fn env_vars() {
        dotenv().ok();

        let mal_cofnig = MALConfig::from_env();

        let response = mal_cofnig
            .get(
                "https://api.myanimelist.net/v2/anime",
                hashmap! {
                    "q" => "Death Note",
                    "limit" => "1"
                },
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
