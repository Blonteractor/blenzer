use super::enums::*;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use std::collections::HashMap;

type JSONDateTime = DateTime<Utc>;

#[derive(Deserialize)]
pub struct SearchResponse<T> {
    pub data: Vec<HashMap<String, T>>,
}

#[derive(Deserialize)]
pub struct BasicMalObject {
    pub id: usize,
    pub title: String,

    #[serde(rename = "main_picture")]
    pub cover_art: Picture,
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
    pub rating: Option<Rating>,
    pub pictures: Option<Vec<Picture>>,
    pub background: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
pub struct Genre {
    pub id: usize,
    pub name: GenreName,
}

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
pub struct Picture {
    pub medium: String,
    pub large: String,
}

#[derive(Deserialize)]
pub struct AlternativeTitles {
    pub synonyms: Vec<String>,
    pub en: String,
    pub ja: String,
}
