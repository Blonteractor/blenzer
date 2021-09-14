pub mod enums;
pub mod structs;

use super::prelude::enums::*;
use super::prelude::*;

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
            .search_anime(query, 1, true)
            .await?
            .json::<SearchResponse<Self>>()
            .await?
            .data[0]
            .drain()
            .next()
            .unwrap()
            .1)
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
        let anime = Anime::from_name("Death Note").await.unwrap();
        assert_eq!(anime.id, 1535);
    }
}
