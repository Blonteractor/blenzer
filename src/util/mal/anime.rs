use chrono::{DateTime, Utc};
use serde::Deserialize;
use std::{collections::HashMap, hash::Hash};

type JSONMap = HashMap<String, String>;
type JSONDateTime = DateTime<Utc>;
#[derive(Deserialize)]
pub struct MangaResponse {
    pub id: usize,
    pub title: String,
    pub medium: JSONMap,
    pub alternative_titles: AlternativeTitles,
    pub start_date: JSONDateTime,
    pub end_date: JSONDateTime,
    pub synopsis: String,
    pub mean: f32,
    pub rank: usize,
    pub popularity: usize,
    pub num_list_users: usize,
    pub scoring_users: usize,
    pub nsfw: NSFWLevel,
    pub created_at: JSONDateTime,
    pub updated_at: JSONDateTime,
    pub media_type: MediaType,
    pub status: Status,
    pub genres: Vec<HashMap<String, Genre>>,
}

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
pub enum Genre {
    Action,
    Adventure,
    Comedy,
    Drama,
    SliceOfLife,
    Fantasy,
    Magic,
    Supernatural,
    Horror,
    Mystery,
    Psychological,
    Romance,
    SciFi,
    Cypberpunk,
    Game,
    Ecchi,
    Demons,
    Harem,
    Josei,
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
    PostApocalyptic,
    ReverseHarem,
    School,
    Seinen,
    Shoujou,
    ShoujoAi,
    Shounen,
    ShounenAi,
    Space,
    Sports,
    SuperPower,
    Tragedy,
    Vampire,
    Yuri,
    Yaoi,
}

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
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
