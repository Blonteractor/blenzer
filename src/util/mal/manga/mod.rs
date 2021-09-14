pub mod enums;
pub mod structs;

use super::prelude::*;
use super::MALClient;
use serde::Deserialize;
use std::ops::Deref;
use structs::*;

pub type MangaSearchResponse = SearchResponse<Manga>;

#[derive(Deserialize)]
pub struct Manga {
    #[serde(flatten)]
    pub data: BasicMalObject,

    #[serde(rename = "num_chapters")]
    pub chapters: usize,

    #[serde(rename = "num_volumes")]
    pub volumes: usize,

    pub authors: Vec<Author>,
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

    pub async fn from_name(query: &str) -> Result<Self, reqwest::Error> {
        Ok(MALClient::from_env()
            .search_manga(query, 1, true)
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
        let manga = Manga::from_name("Blame").await.unwrap();

        assert_eq!(manga.id, 149);
        assert_eq!(manga.title, "Blame!");
    }
}
