use serde::Deserialize;
use std::fmt::Display;

#[derive(enum_display_derive::Display, Deserialize)]
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
    RX,
    NA,
}

#[derive(enum_display_derive::Display, Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
pub enum DayOfTheWeek {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
    NA,
}

#[derive(enum_display_derive::Display, Deserialize)]
pub enum GenreName {
    Action,
    Adventure,
    Comedy,
    Drama,
    #[serde(rename = "Slice of Life")]
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
    Military,
    #[serde(rename = "Boys Love")]
    BoysLove,
    Erotica,
    Harem,
    Josei,
    #[serde(rename = "Martial Arts")]
    MartialArts,
    #[serde(rename = "Avant Garde")]
    AvantGarde,
    Kids,
    Historical,
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
    #[serde(rename = "Girls Love")]
    GirlsLove,
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
    Suspense,
    #[serde(rename = "Gender Bender")]
    GenderBender,
    NA,
}

#[derive(enum_display_derive::Display, Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
pub enum Status {
    FinishedAiring,
    CurrentlyAiring,
    CurrentlyPublishing,
    Finished,
    OnHiatus,
    NotYetAired,
    NA,
}

#[derive(enum_display_derive::Display, Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
pub enum NSFWLevel {
    White,
    Black,
    Gray,
    NA,
}

#[derive(enum_display_derive::Display, Deserialize)]
#[serde(rename_all(deserialize = "lowercase"))]
pub enum MediaType {
    TV,
    ONA,
    OVA,
    Movie,
    Special,
    Manga,
    Manhwa,
    Novel,

    #[serde(rename = "light_novel")]
    LightNovel,

    #[serde(rename = "one_shot")]
    OneShot,
    NA,
}
