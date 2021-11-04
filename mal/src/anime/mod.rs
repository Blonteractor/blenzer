pub mod enums;
pub mod structs;

use super::prelude::enums::*;
use super::prelude::*;

use super::prelude::Reloadable;
use super::MALClient;
use async_trait::async_trait;
use enums::*;
use serde::Deserialize;
use std::ops::Deref;
use structs::*;

#[derive(Deserialize)]
pub struct Anime {
    #[serde(flatten)]
    pub data: BasicMalObject,

    #[serde(rename = "num_episodes")]
    pub episodes: Option<usize>,
    pub start_season: Option<StartSeason>,
    pub broadcast: Option<Broadcast>,
    pub source: Option<Source>,
    pub rating: Option<Rating>,
    pub studios: Option<Vec<Studio>>,

    #[serde(rename = "average_episode_duration")]
    pub episode_duration: Option<usize>,
}

impl Anime {
    pub async fn from_id(id: usize) -> Result<Self, reqwest::Error> {
        MALClient::from_env()
            .get_anime_id(id)
            .await?
            .json::<Self>()
            .await
    }

    pub async fn from_name(query: &str, nsfw: bool) -> Result<Self, reqwest::Error> {
        Ok(MALClient::from_env()
            .search_anime(query, Some(1), None, nsfw, true)
            .await?
            .json::<SearchResponse<Self>>()
            .await?
            .data[0]
            .drain()
            .next()
            .unwrap()
            .1)
    }

    pub async fn reload(&mut self) {
        *self = Self::from_id(self.id).await.unwrap();
    }

    pub fn url(&self) -> String {
        format!(
            "https://myanimelist.net/anime/{}/{}",
            self.id,
            self.title.replace(" ", "_")
        )
    }

    pub async fn search_basic(
        query: &str,
        limit: usize,
        nsfw: bool,
    ) -> Result<Vec<Self>, reqwest::Error> {
        Ok(MALClient::from_env()
            .get(
                "https://api.myanimelist.net/v2/anime",
                &hashmap! {
                    "q" => query.to_string(),
                    "fields" => "title".to_string(),
                    "nsfw" => nsfw.to_string(),
                    "limit" => limit.to_string()
                },
            )
            .await
            .unwrap()
            .json::<SearchResponse<Self>>()
            .await?
            .data
            .into_iter()
            .map(|result| result.into_values().next().unwrap())
            .collect::<Vec<Self>>())
    }
}

#[async_trait]
impl Reloadable for Anime {
    async fn reload(&mut self) {
        *self = Self::from_id(self.id).await.unwrap();
    }
}
impl Deref for Anime {
    type Target = BasicMalObject;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

#[cfg(test)]
mod test {
    use super::Anime;

    #[tokio::test]
    async fn from_id() {
        let anime = Anime::from_id(30230).await.unwrap();

        assert_eq!(anime.id, 30230);
    }

    #[tokio::test]
    async fn from_name() {
        let _ = Anime::from_name("Death Note", true).await.unwrap();
    }

    #[tokio::test]
    async fn reload() {
        let mut anime = Anime::from_name("Death Note", true).await.unwrap();
        let old_id = anime.id;
        anime.reload().await;
        assert_eq!(anime.id, old_id);
    }

    #[tokio::test]
    async fn search_basic() {
        let results = Anime::search_basic("highschool", 6, true).await.unwrap();

        assert_eq!(results.len(), 6);
    }
}
