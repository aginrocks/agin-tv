use crate::{models::Movie, tmdb_configuration::MovieDetailsResponse};
use chrono::Utc;
use mongodb::bson::{self, Document, oid::ObjectId};
use serde::Serialize;
use tmdb::models::TvSeriesDetails200Response;

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
        if let Some(stripped) = s.strip_prefix('m') {
            Ok(TMDBMovieId::Movie(stripped.to_string()))
        } else if let Some(stripped) = s.strip_prefix('t') {
            Ok(TMDBMovieId::TVShow(stripped.to_string()))
        } else if let Some(stripped) = s.strip_prefix('c') {
            Ok(TMDBMovieId::Custom(stripped.to_string()))
        } else {
            Err(serde::de::Error::custom("Invalid TMDB movie ID format"))
        }
    }
}

pub enum TMDBMovieData {
    Movie(MovieDetailsResponse),
    TV(TvSeriesDetails200Response),
}

impl Movie {
    fn parse_date_string(date_str: &str) -> Option<chrono::DateTime<Utc>> {
        // Parse date string in format "YYYY-MM-DD"
        let parts: Vec<&str> = date_str.split('-').collect();
        if parts.len() != 3 {
            return None;
        }

        let year: i32 = parts[0].parse().ok()?;
        let month: u32 = parts[1].parse().ok()?;
        let day: u32 = parts[2].parse().ok()?;

        let dt = chrono::NaiveDate::from_ymd_opt(year, month, day)?
            .and_hms_opt(0, 0, 0)?
            .and_utc();
        Some(dt)
    }

    pub fn from_tmdb(movie: TMDBMovieData) -> Self {
        match movie {
            TMDBMovieData::Movie(movie) => {
                // Find the best logo
                let logo_url = movie
                    .images
                    .clone()
                    .and_then(|images| images.logos)
                    .and_then(|logos| {
                        if logos.is_empty() {
                            return None;
                        }

                        // Try to find English logo first
                        if let Some(logo) =
                            logos.iter().find(|l| l.iso_639_1 == Some("en".to_string()))
                        {
                            return logo.file_path.clone();
                        }

                        // Try to find logo in original language
                        if let Some(original_language) = &movie.original_language
                            && let Some(logo) = logos
                                .iter()
                                .find(|l| l.iso_639_1 == Some(original_language.to_string()))
                        {
                            return logo.file_path.clone();
                        }

                        // Try to find logo with no language specified
                        if let Some(logo) = logos.iter().find(|l| l.iso_639_1.is_none()) {
                            return logo.file_path.clone();
                        }

                        // Fall back to first logo
                        logos.first().and_then(|logo| logo.file_path.clone())
                    })
                    .map(|path| format!("https://image.tmdb.org/t/p/original{path}"));

                // The same logic for horizontal cover
                let horizontal_cover_url = movie
                    .images
                    .and_then(|images| images.backdrops)
                    .and_then(|backdrops| {
                        if backdrops.is_empty() {
                            return None;
                        }

                        if let Some(backdrop) = backdrops
                            .iter()
                            .find(|l| l.iso_639_1 == Some("en".to_string()))
                        {
                            return backdrop.file_path.clone();
                        }

                        if let Some(original_language) = &movie.original_language
                            && let Some(backdrop) = backdrops
                                .iter()
                                .find(|l| l.iso_639_1 == Some(original_language.to_string()))
                        {
                            return backdrop.file_path.clone();
                        }

                        backdrops
                            .first()
                            .and_then(|backdrop| backdrop.file_path.clone())
                    })
                    .map(|path| format!("https://image.tmdb.org/t/p/original{path}"));

                Self {
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
                    background_url: movie
                        .backdrop_path
                        .map(|p| format!("https://image.tmdb.org/t/p/original{p}")),
                    horizontal_cover_url,
                    logo_url,
                    original_language: movie.original_language,
                }
            }
            TMDBMovieData::TV(_tv) => Self {
                ..Default::default()
            },
        }
    }
    pub fn from_database(movie: Movie) -> Self {
        Self {
            id: movie.id,
            tmdb_id: movie.tmdb_id,
            tv: movie.tv,
            name: movie.name,
            original_name: movie.original_name,
            description: movie.description,
            release_date: movie.release_date,
            genres: movie.genres,
            vertical_cover_url: movie.vertical_cover_url,
            horizontal_cover_url: movie.horizontal_cover_url,
            background_url: movie.background_url,
            logo_url: movie.logo_url,
            original_language: movie.original_language,
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
