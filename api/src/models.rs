use crate::mongo_id::{object_id_as_string_required, vec_oid_to_vec_string};
use mongodb::bson::{self, DateTime, Document, oid::ObjectId};
use partial_struct::Partial;
use serde::{Deserialize, Serialize};
use tmdb::models::{MovieDetails200Response, TvSeriesDetails200Response};
use utoipa::ToSchema;
use visible::StructFields;

macro_rules! database_object {
    ($name:ident { $($field:tt)* }$(, $($omitfield:ident),*)?) => {
        #[derive(Partial, Debug, Serialize, Deserialize, ToSchema, Clone, Default)]
        #[partial(omit(id $(, $($omitfield),* )?), derive(Debug, Serialize, Deserialize, ToSchema, Clone))]
        #[StructFields(pub)]
        pub struct $name {
            $($field)*

        }
    };
}

database_object!(User {
    #[serde(rename = "_id", with = "object_id_as_string_required")]
    #[schema(value_type = String)]
    id: ObjectId,
    subject: String,
    name: String,
    email: String,
});

database_object!(
    Movie {
    #[serde(rename = "_id", with = "object_id_as_string_required")]
    #[schema(value_type = String)]
    id: ObjectId,
    #[schema(value_type = String)]
    tmdb_id: TMDBMovieId,
    name: String,
    original_name: Option<String>,
    description: String,
    tv: bool,
    #[schema(value_type = String)]
    release_date: Option<DateTime>,
    vertical_cover_url: Option<String>,
    horizontal_cover_url: Option<String>,
    background_url: Option<String>,
    logo_url: Option<String>,
    #[serde(with = "vec_oid_to_vec_string")]
    #[schema(value_type = Vec<String>)]
    genres: Vec<ObjectId>,

});

struct XD {}

#[derive(Debug, Clone)]
pub enum TMDBMovieId {
    Movie(String),
    TVShow(String),
    Custom(String),
}

impl std::fmt::Display for TMDBMovieId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TMDBMovieId::Movie(id) => write!(f, "m{id}"),
            TMDBMovieId::TVShow(id) => write!(f, "t{id}"),
            TMDBMovieId::Custom(id) => write!(f, "c{id}"),
        }
    }
}

impl Default for TMDBMovieId {
    fn default() -> Self {
        Self::Movie("0".to_string())
    }
}

impl Serialize for TMDBMovieId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let prefix = match self {
            TMDBMovieId::Movie(_) => "m",
            TMDBMovieId::TVShow(_) => "t",
            TMDBMovieId::Custom(_) => "c",
        };

        let id = match self {
            TMDBMovieId::Movie(id) => id,
            TMDBMovieId::TVShow(id) => id,
            TMDBMovieId::Custom(id) => id,
        };

        format!("{prefix}{id}").serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for TMDBMovieId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s.starts_with('m') {
            Ok(TMDBMovieId::Movie(s[1..].to_string()))
        } else if s.starts_with('t') {
            Ok(TMDBMovieId::TVShow(s[1..].to_string()))
        } else if s.starts_with('c') {
            Ok(TMDBMovieId::Custom(s[1..].to_string()))
        } else {
            Err(serde::de::Error::custom("Invalid TMDB movie ID format"))
        }
    }
}

pub enum TMDBMovieData {
    Movie(MovieDetails200Response),
    TV(TvSeriesDetails200Response),
}

impl Movie {
    fn parse_date_string(date_str: &str) -> Option<DateTime> {
        // Parse date string in format "YYYY-MM-DD"
        let parts: Vec<&str> = date_str.split('-').collect();
        if parts.len() != 3 {
            return None;
        }

        let year: i32 = parts[0].parse().ok()?;
        let month: u32 = parts[1].parse().ok()?;
        let day: u32 = parts[2].parse().ok()?;

        // Create a DateTime for the start of the day (midnight UTC)
        let timestamp_millis = chrono::NaiveDate::from_ymd_opt(year, month, day)?
            .and_hms_opt(0, 0, 0)?
            .and_utc()
            .timestamp_millis();

        Some(DateTime::from_millis(timestamp_millis))
    }

    pub fn from_tmdb(movie: TMDBMovieData) -> Self {
        match movie {
            TMDBMovieData::Movie(movie) => Self {
                id: ObjectId::new(),
                tmdb_id: TMDBMovieId::Movie(movie.id.expect("XD").to_string()),
                tv: false,
                name: movie.title.unwrap_or_default(),
                original_name: movie.original_title,
                description: movie.overview.unwrap_or_default(),
                release_date: movie
                    .release_date
                    .and_then(|date| Self::parse_date_string(&date)),
                genres: vec![],
                vertical_cover_url: movie
                    .poster_path
                    .map(|p| format!("https://image.tmdb.org/t/p/w500{p}")),
                ..Default::default()
            },
            TMDBMovieData::TV(tv) => Self {
                ..Default::default()
            },
        }
    }
}

impl From<Movie> for Document {
    fn from(val: Movie) -> Self {
        let mut doc = bson::to_document(&val).expect("Failed to convert Movie to Document");
        doc.remove("_id");
        doc
    }
}
