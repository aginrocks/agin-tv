use crate::{
    models::{Genre, Movie},
    routes::movies::movie_id::MovieResponse,
    state::AppState,
    tmdb_configuration::{MovieDetailsResponse, TvDetailsResponse},
};
use bson::doc;
use chrono::Utc;
use futures::stream::TryStreamExt;
use mongodb::bson::{self, Document, oid::ObjectId};
use serde::Serialize;
use tmdb::models::MovieDetails200ResponseGenresInner;

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

#[derive(Debug, Clone)]
pub enum TMDBMovieData {
    Movie(MovieDetailsResponse),
    TV(TvDetailsResponse),
}

macro_rules! extract_image_url {
    ($images:expr, $field:ident, $original_language:expr) => {
        $images
            .clone()
            .and_then(|images| images.$field)
            .and_then(|items| {
                if items.is_empty() {
                    return None;
                }

                // Try to find English item first
                if let Some(item) = items.iter().find(|l| l.iso_639_1 == Some("en".to_string())) {
                    return item.file_path.clone();
                }

                // Try to find item in original language
                if let Some(original_language) = $original_language
                    && let Some(item) = items
                        .iter()
                        .find(|l| l.iso_639_1 == Some(original_language.to_string()))
                {
                    return item.file_path.clone();
                }

                // Try to find item with no language specified
                if let Some(item) = items.iter().find(|l| l.iso_639_1.is_none()) {
                    return item.file_path.clone();
                }

                // Fall back to first item
                items.first().and_then(|item| item.file_path.clone())
            })
            .map(|path| format!("https://image.tmdb.org/t/p/original{path}"))
    };
}

pub async fn add_genres_to_database(
    genres: Vec<MovieDetails200ResponseGenresInner>,
    state: AppState,
) -> color_eyre::Result<Vec<ObjectId>> {
    let mut genre_ids = Vec::new();
    for genre in genres {
        let results = state
            .db
            .collection::<Genre>("genres")
            .find_one_and_update(
                doc! {"tmdb_id": genre.id},
                doc! {"$set": {"name": genre.name}},
            )
            .upsert(true)
            .return_document(mongodb::options::ReturnDocument::After)
            .await?;

        if let Some(g) = results {
            genre_ids.push(g.id)
        }
    }
    Ok(genre_ids)
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

    pub async fn from_tmdb(movie: TMDBMovieData, state: AppState) -> color_eyre::Result<Self> {
        match movie {
            TMDBMovieData::Movie(movie) => {
                let logo_url = extract_image_url!(movie.images, logos, &movie.original_language);
                let horizontal_cover_url =
                    extract_image_url!(movie.images, backdrops, &movie.original_language);

                let genres = if let Some(genres) = movie.genres {
                    tracing::info!("Adding genres to database: {:?}", genres);
                    add_genres_to_database(genres, state.clone()).await?
                } else {
                    tracing::info!("No genres found for movie");
                    vec![]
                };

                Ok(Self {
                    id: ObjectId::new(),
                    tmdb_id: TMDBMovieId::Movie(movie.id.expect("XD").to_string()),
                    tv: false,
                    name: movie.title.unwrap_or_default(),
                    original_name: movie.original_title,
                    description: movie.overview.unwrap_or_default(),
                    release_date: movie
                        .release_date
                        .and_then(|date| Self::parse_date_string(&date)),
                    genres,
                    vertical_cover_url: movie
                        .poster_path
                        .map(|p| format!("https://image.tmdb.org/t/p/original{p}")),
                    background_url: movie
                        .backdrop_path
                        .map(|p| format!("https://image.tmdb.org/t/p/original{p}")),
                    horizontal_cover_url,
                    logo_url,
                    original_language: movie.original_language,
                })
            }
            TMDBMovieData::TV(tv) => {
                let logo_url = extract_image_url!(tv.images, logos, &tv.original_language);
                let horizontal_cover_url =
                    extract_image_url!(tv.images, backdrops, &tv.original_language);

                let genres = if let Some(genres) = tv.genres {
                    add_genres_to_database(genres, state).await?
                } else {
                    vec![]
                };

                Ok(Self {
                    id: ObjectId::new(),
                    tmdb_id: TMDBMovieId::TVShow(tv.id.expect("XD").to_string()),
                    tv: true,
                    name: tv.name.unwrap_or_default(),
                    original_name: tv.original_name,
                    description: tv.overview.unwrap_or_default(),
                    release_date: tv
                        .first_air_date
                        .and_then(|date| Self::parse_date_string(&date)),
                    genres,
                    vertical_cover_url: tv
                        .poster_path
                        .map(|p| format!("https://image.tmdb.org/t/p/original{p}")),
                    background_url: tv
                        .backdrop_path
                        .map(|p| format!("https://image.tmdb.org/t/p/original{p}")),
                    horizontal_cover_url,
                    logo_url,
                    original_language: tv.original_language,
                })
            }
        }
    }
    pub async fn populate_genres(self, state: AppState) -> color_eyre::Result<MovieResponse> {
        let genres = state
            .db
            .collection::<Genre>("genres")
            .find(
                doc! {"_id": {"$in": self.genres.iter().map(|g| g.to_owned()).collect::<Vec<_>>() }},
            )
            .await?;

        let genres: Vec<Genre> = genres.try_collect().await?;

        Ok(MovieResponse {
            _id: self.id.to_string(),
            tmdb_id: self.tmdb_id.to_string(),
            name: self.name.clone(),
            original_name: self.original_name.clone(),
            description: self.description.clone(),
            tv: self.tv,
            release_date: self.release_date,
            vertical_cover_url: self.vertical_cover_url.clone(),
            horizontal_cover_url: self.horizontal_cover_url.clone().clone(),
            background_url: self.background_url.clone().clone(),
            logo_url: self.logo_url.clone().clone(),
            genres,
            original_language: self.original_language.clone(),
        })
    }
}

impl From<Movie> for Document {
    fn from(val: Movie) -> Self {
        let mut doc = bson::to_document(&val).expect("Failed to convert Movie to Document");
        doc.remove("_id");
        doc
    }
}
