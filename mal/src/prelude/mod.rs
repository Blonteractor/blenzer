pub mod enums;
pub mod structs;

use chrono::{DateTime, Utc};
use enums::*;
use serde::Deserialize;
use std::collections::HashMap;
use structs::*;

type JSONDateTime = DateTime<Utc>;
use async_trait::async_trait;

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
    pub genres: Option<Vec<Genre>>,
    pub rating: Option<Rating>,
    pub pictures: Option<Vec<Picture>>,
    pub background: Option<String>,
}

#[derive(Deserialize)]
pub struct SearchResponse<T> {
    pub data: Vec<HashMap<String, T>>,
}

impl<T> std::ops::Deref for SearchResponse<T> {
    type Target = Vec<HashMap<String, T>>;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

#[async_trait]
pub trait Reloadable {
    async fn reload(&mut self);
}
