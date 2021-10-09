pub mod enums;
pub mod structs;

use super::prelude::enums::*;
use super::prelude::*;

use super::MALClient;
use enums::*;
use futures;
use serde::Deserialize;
use std::collections::HashMap;
use std::ops::Deref;
use structs::*;

// use futures::Future;
// use futures::FutureExt;
// use std::pin::Pin;
// use std::task::Context;
// use std::task::Poll;

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
    // pub async fn search_basic(
    //     query: &str,
    //     limit: usize,
    //     nsfw: bool,
    // ) -> Result<AnimeBasicSearch, reqwest::Error> {
    //     Ok(AnimeBasicSearch::start(query.to_string(), limit, nsfw))
    // }
}

pub struct AnimeBasicSearch {
    #[allow(dead_code)]
    current_offset: usize,
    #[allow(dead_code)]
    client: MALClient,
    #[allow(dead_code)]
    limit: usize,
    #[allow(dead_code)]
    params: HashMap<&'static str, String>,
}

impl AnimeBasicSearch {
    pub fn start(query: String, limit: usize, nsfw: bool) -> Self {
        let params = hashmap! {
            "q" => query,
            "limit" => 1.to_string(),
            "nsfw" => nsfw.to_string(),
            "offset" => 0.to_string()
        };
        Self {
            current_offset: 0,
            client: MALClient::from_env(),
            params,
            limit,
        }
    }
}

// impl futures::stream::Stream for AnimeBasicSearch {
//     type Item = (usize, String);

//     fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
//         let x = self.client.get("", &self.params);
//         let mut result = futures::pin_mut!(x);
//         match result.as_mut().poll(cx) {
//             Poll::Ready(e) => Poll::Ready(None),
//             Poll::Pending => Poll::Pending,
//         }
//     }
// }

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
}
