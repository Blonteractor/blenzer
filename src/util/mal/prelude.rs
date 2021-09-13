pub mod enums {
    use serde::Deserialize;

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
    pub enum Status {
        FinishedAiring,
        Airing,
        CurrentlyPublishing,
    }

    #[derive(Deserialize)]
    #[serde(rename_all(deserialize = "snake_case"))]
    pub enum NSFWLevel {
        White,
        Black,
    }

    #[derive(Deserialize)]
    #[serde(rename_all(deserialize = "lowercase"))]
    pub enum MediaType {
        TV,
        ONA,
        OVA,
        Movie,
        Special,
        Manga,
    }
}

pub mod structs {
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
}
