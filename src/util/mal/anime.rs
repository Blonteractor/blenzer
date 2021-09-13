use chrono::{DateTime, Utc};
use serde::Deserialize;
use std::collections::HashMap;

type JSONMap = HashMap<String, String>;
type JSONDateTime = DateTime<Utc>;

use super::prelude::enums::*;
use super::prelude::structs::*;

#[derive(Deserialize)]
pub struct AnimeSearchResponse {
    pub data: Vec<HashMap<String, Anime>>,
}
#[derive(Deserialize)]
pub struct Anime {
    pub id: usize,
    pub title: String,

    #[serde(rename = "main_picture")]
    pub cover_art: Picture,
    pub medium: Option<JSONMap>,
    pub alternative_titles: Option<AlternativeTitles>,

    #[serde(rename = "start_date")]
    pub start: Option<String>,

    #[serde(rename = "end_date")]
    pub end: Option<String>,
    pub synopsis: Option<String>,

    #[serde(rename = "mean")]
    pub score: Option<f32>,

    pub rank: Option<usize>,
    pub popularity: Option<usize>,
    pub num_list_users: Option<usize>,
    pub scoring_users: Option<usize>,
    pub nsfw: Option<NSFWLevel>,
    pub created_at: Option<JSONDateTime>,
    pub updated_at: Option<JSONDateTime>,
    pub media_type: Option<MediaType>,
    pub status: Option<Status>,
    pub genres: Vec<Genre>,

    #[serde(rename = "num_episodes")]
    pub episodes: Option<usize>,
    pub start_season: Option<StartSeason>,
    pub broadcast: Option<Broadcast>,
    pub source: Option<Source>,

    #[serde(rename = "average_episode_duration")]
    pub episode_duration: Option<usize>,
    pub rating: Option<Rating>,
    pub pictures: Option<Vec<Picture>>,
    pub studios: Vec<Studio>,
}

#[cfg(test)]
mod test {
    use super::{super::config::MALConfig, Anime};
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

        let manga = response.json::<Anime>().await.unwrap();

        assert_eq!(manga.id, 30230);
        assert_eq!(manga.title, "Diamond no Ace: Second Season");
    }
}
