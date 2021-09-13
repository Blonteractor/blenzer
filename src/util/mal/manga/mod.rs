pub mod enums;
pub mod structs;

use super::prelude::structs::*;
use super::prelude::*;
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

#[cfg(test)]
mod test {
    use super::{super::MALConfig, Manga};
    #[tokio::test]

    async fn berserk() {
        let mal_config = MALConfig::from_env();

        let response = mal_config
            .get(
                "https://api.myanimelist.net/v2/manga/2",
                hashmap! {
                    "fields" => "title,main_picture,alternative_titles,start_date,end_date,synopsis,mean,rank,popularity,nsfw,created_at,media_type,status,genres,pictures,background,studios,num_volumes,num_chapters,authors"
                },
            )
            .await
            .unwrap();

        let manga = response.json::<Manga>().await.unwrap();

        assert_eq!(manga.id, 2);
        assert_eq!(manga.title, "Berserk");
    }
}
