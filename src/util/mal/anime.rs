use chrono::{DateTime, Utc};
use serde::Deserialize;
use std::collections::HashMap;

type JSONMap = HashMap<String, String>;
type JSONDateTime = DateTime<Utc>;

#[derive(Deserialize)]
pub struct MangaSearchResponse {
    pub data: Vec<HashMap<String, Manga>>,
}
#[derive(Deserialize)]
pub struct Manga {
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

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
pub struct Genre {
    pub id: usize,
    pub name: GenreName,
}

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
pub struct Studio {
    pub id: usize,
    pub name: String,
}

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
pub struct Picture {
    pub medium: String,
    pub large: String,
}

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "lowercase"))]
pub enum Rating {
    #[serde(rename = "pg_13")]
    PG13,
    #[serde(rename = "r")]
    R,
    #[serde(rename = "r+")]
    RPLUS,
    #[serde(rename = "g")]
    G,
    #[serde(rename = "pg")]
    PG,
}

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
pub struct Broadcast {
    pub day_of_the_week: DayOfTheWeek,
    pub start_time: String,
}

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
pub enum DayOfTheWeek {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
pub struct StartSeason {
    pub year: usize,
    pub season: Season,
}

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
pub enum Season {
    Spring,
    Summer,
    Fall,
    Winter,
}

#[derive(Deserialize)]
pub enum GenreName {
    Action,
    Adventure,
    Comedy,
    Drama,
    #[serde(rename = "Slice Of Life")]
    SliceOfLife,
    Fantasy,
    Magic,
    Supernatural,
    Horror,
    Mystery,
    Psychological,
    Romance,
    #[serde(rename = "Sci-Fi")]
    SciFi,
    Cypberpunk,
    Game,
    Ecchi,
    Demons,
    Harem,
    Josei,
    #[serde(rename = "Martial Arts")]
    MartialArts,
    Kids,
    Hisorical,
    Hentai,
    Isekai,
    Millitary,
    Mecha,
    Music,
    Parody,
    Police,
    #[serde(rename = "Post Apocalyptic")]
    PostApocalyptic,
    #[serde(rename = "Reverse Harem")]
    ReverseHarem,
    School,
    Seinen,
    Shoujou,
    Samurai,
    #[serde(rename = "Shoujou Ai")]
    ShoujoAi,
    Shounen,
    #[serde(rename = "Shounen Ai")]
    ShounenAi,
    Space,
    Sports,
    #[serde(rename = "Super Power")]
    SuperPower,
    Tragedy,
    Vampire,
    Yuri,
    Yaoi,
    Thriller,
}

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
#[serde(rename_all(deserialize = "lowercase"))]
pub enum MediaType {
    TV,
    ONA,
    OVA,
    Movie,
    Special,
}

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
pub enum Status {
    FinishedAiring,
    Airing,
}

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
pub enum NSFWLevel {
    White,
    Black,
}
#[derive(Deserialize)]
pub struct AlternativeTitles {
    pub synonyms: Vec<String>,
    pub en: String,
    pub ja: String,
}

#[cfg(test)]
mod test {
    use super::{super::prelude::MALConfig, Manga};
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

        let manga = response.json::<Manga>().await.unwrap();

        assert_eq!(manga.id, 30230);
        assert_eq!(manga.title, "Diamond no Ace: Second Season");
    }
}
