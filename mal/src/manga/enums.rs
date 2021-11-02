use serde::Deserialize;
use std::fmt::Display;

#[derive(Deserialize, enum_display_derive::Display)]
pub enum AuthorRole {
    Art,
    Story,

    #[serde(rename = "Story & Art")]
    ArtStory,
}
