pub mod enums;
pub mod structs;

use super::prelude::enums::*;
use super::prelude::*;
use reqwest::Response;

use super::MALClient;
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

    pub async fn from_name(query: &str) -> Result<Self, reqwest::Error> {
        Ok(MALClient::from_env()
            .search_anime(query, Some(1), None, true)
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
    pub async fn search_basic(
        query: &str,
        limit: usize,
    ) -> Result<AnimeBasicSearch, reqwest::Error> {
        Ok(AnimeBasicSearch::start(
            MALClient::from_env()
                .search_anime(query, Some(limit), None, false)
                .await?,
        ))
    }
}

pub struct AnimeBasicSearch {
    data: Response,
}

impl AnimeBasicSearch {
    pub fn start(response: Response) -> Self {
        Self { data: response }
    }
}

impl Iterator for AnimeBasicSearch {
    type Item = (isize, String);

    fn next(&mut self) -> Option<Self::Item> {
        None
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
        let _ = Anime::from_name("Death Note").await.unwrap();
    }

    #[tokio::test]
    async fn reload() {
        let mut anime = Anime::from_name("Death Note").await.unwrap();
        let old_id = anime.id;
        anime.reload().await;
        assert_eq!(anime.id, old_id);
    }
}
