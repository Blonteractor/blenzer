use serde::Deserialize;
use std::fmt::Display;

#[derive(enum_display_derive::Display, Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
pub enum Source {
    Original,
    Manga,
    Game,
    LightNovel,
    VisualNovel,
    NA,
}

#[derive(enum_display_derive::Display, Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
pub enum Season {
    Spring,
    Summer,
    Fall,
    Winter,
    NA,
}
