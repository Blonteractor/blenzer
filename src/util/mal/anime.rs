use super::prelude::enums::*;
use super::prelude::structs::*;
use serde::Deserialize;
use std::ops::Deref;

pub type AnimeSearchResponse = SearchResponse<Anime>;

use enums::*;
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
    pub pictures: Option<Vec<Picture>>,
    pub background: Option<String>,
}

impl Deref for Anime {
    type Target = BasicMalObject;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

pub mod structs {
    use super::super::prelude::enums::*;
    use super::enums::*;
    use serde::Deserialize;

    #[derive(Deserialize)]
    #[serde(rename_all(deserialize = "snake_case"))]
    pub struct Studio {
        pub id: usize,
        pub name: String,
    }

    #[derive(Deserialize)]
    #[serde(rename_all(deserialize = "snake_case"))]
    pub struct Broadcast {
        pub day_of_the_week: DayOfTheWeek,
        pub start_time: String,
    }

    #[derive(Deserialize)]
    #[serde(rename_all(deserialize = "snake_case"))]
    pub struct StartSeason {
        pub year: usize,
        pub season: Season,
    }
}
pub mod enums {
    use serde::Deserialize;

    #[derive(Deserialize)]
    #[serde(rename_all(deserialize = "snake_case"))]
    pub enum Source {
        Orignal,
        Manga,
        Game,
        LightNovel,
        VisualNovel,
    }

    #[derive(Deserialize)]
    #[serde(rename_all(deserialize = "snake_case"))]
    pub enum Season {
        Spring,
        Summer,
        Fall,
        Winter,
    }
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

        let anime = response.json::<Anime>().await.unwrap();

        assert_eq!(anime.id, 30230);
        assert_eq!(anime.title, "Diamond no Ace: Second Season");
    }
}
