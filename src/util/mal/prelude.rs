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
}

pub mod structs {
    use super::enums::*;
    use serde::Deserialize;

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

    #[derive(Deserialize)]
    pub struct AlternativeTitles {
        pub synonyms: Vec<String>,
        pub en: String,
        pub ja: String,
    }
}
