pub mod enums;
pub mod structs;

use super::prelude::*;
use super::MALClient;
use async_trait::async_trait;
use serde::Deserialize;
use std::ops::Deref;
use structs::*;

pub type MangaSearchResponse = SearchResponse<Manga>;

#[derive(Deserialize)]
pub struct Manga {
    #[serde(flatten)]
    pub data: BasicMalObject,

    #[serde(rename = "num_chapters")]
    pub chapters: Option<usize>,

    #[serde(rename = "num_volumes")]
    pub volumes: Option<usize>,

    pub authors: Option<Vec<Author>>,
}

impl Deref for Manga {
    type Target = BasicMalObject;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl Manga {
    pub async fn from_id(id: usize) -> Result<Self, reqwest::Error> {
        MALClient::from_env()
            .get_manga_id(id)
            .await?
            .json::<Self>()
            .await
    }

    pub async fn from_name(query: &str, nsfw: bool) -> Result<Self, reqwest::Error> {
        Ok(MALClient::from_env()
            .search_manga(query, Some(1), None, nsfw, true)
            .await?
            .json::<SearchResponse<Self>>()
            .await?
            .data[0]
            .drain()
            .next()
            .unwrap()
            .1)
    }

    pub fn url(&self) -> String {
        format!(
            "https://myanimelist.net/manga/{}/{}",
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
                "https://api.myanimelist.net/v2/manga",
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

    pub async fn reload(&mut self) {
        *self = Self::from_id(self.id).await.unwrap();
    }
}

#[async_trait]
impl Reloadable for Manga {
    async fn reload(&mut self) {
        *self = Self::from_id(self.id).await.unwrap();
    }
}

#[cfg(test)]
mod test {
    use super::Manga;

    #[tokio::test]
    async fn from_id() {
        let manga = Manga::from_id(2).await.unwrap();

        assert_eq!(manga.id, 2);
        assert_eq!(manga.title, "Berserk");
    }

    #[tokio::test]
    async fn from_name() {
        let _ = Manga::from_name("Blame", true).await.unwrap();
    }

    #[tokio::test]
    async fn reload() {
        let mut manga = Manga::from_name("Death Note", true).await.unwrap();
        let old_id = manga.id;
        manga.reload().await;
        assert_eq!(manga.id, old_id);
    }

    #[tokio::test]
    async fn search_basic() {
        let results = Manga::search_basic("highschool", 6, true).await.unwrap();

        assert_eq!(results.len(), 6);
    }
}
