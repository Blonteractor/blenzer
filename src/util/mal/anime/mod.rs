pub mod enums;
pub mod structs;

use super::prelude::enums::*;
use super::prelude::structs::*;
use enums::*;
use serde::Deserialize;
use std::ops::Deref;
use structs::*;

pub type AnimeSearchResponse = SearchResponse<Anime>;

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
    pub pictures: Option<Vec<Picture>>,
    pub background: Option<String>,
}

impl Deref for Anime {
    type Target = BasicMalObject;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

#[cfg(test)]
mod test {
    use super::{super::MALConfig, Anime};
    #[tokio::test]

    async fn diamon_no_ace() {
        let mal_cofnig = MALConfig::from_env();

        let response = mal_cofnig
            .get(
                "https://api.myanimelist.net/v2/anime/30230",
                hashmap! {
                    "fields" => "title,main_picture,alternative_titles,start_date,end_date,synopsis,mean,rank,popularity,nsfw,created_at,media_type,status,genres,num_episodes,start_season,broadcast,source,average_episode_duration,rating,pictures,background,studios"
                },
            )
            .await
            .unwrap();

        let anime = response.json::<Anime>().await.unwrap();

        assert_eq!(anime.id, 30230);
        assert_eq!(anime.title, "Diamond no Ace: Second Season");
    }
}
