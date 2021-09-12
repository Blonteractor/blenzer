use chrono::{DateTime, Utc};
use serde::Deserialize;
use std::collections::HashMap;

type JSONMap = HashMap<String, String>;
type JSONDateTime = DateTime<Utc>;
#[derive(Deserialize)]
pub struct MangaResponse {
    pub id: u16,
    pub title: String,
    pub medium: JSONMap,
    pub alternative_titles: AlternativeTitles,
    pub start_date: JSONDateTime,
    pub end_date: JSONDateTime,
    pub synopsis: String,
}

#[derive(Deserialize)]
pub struct AlternativeTitles {
    synonyms: Vec<String>,
    en: String,
    ja: String,
}
