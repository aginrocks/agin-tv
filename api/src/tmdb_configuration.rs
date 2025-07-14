use std::{env, sync::LazyLock};

use serde::{Deserialize, Serialize, de::Error};
use tmdb::{
    apis::{
        ContentType, Error as Err, ResponseContent,
        configuration::{self, Configuration},
        default_api::{MovieDetailsError, TvSeriesDetailsError},
    },
    models,
};

#[allow(clippy::expect_used)]
pub static TMDB_CONFIGURATION: LazyLock<Configuration> = LazyLock::new(|| Configuration {
    base_path: "https://api.themoviedb.org/".to_owned(),
    user_agent: Some("agin-tv-api/1.0.0".to_owned()),
    client: reqwest::ClientBuilder::new()
        .build()
        .expect("error creating reqwest client"),
    basic_auth: None,
    oauth_access_token: None,
    bearer_access_token: Some(std::env::var("TMDB_API_KEY").expect("no TMDB_API_KEY")),
    api_key: Some(configuration::ApiKey {
        key: env::var("TMDB_API_KEY").expect("no TMDB_API_KEY"),
        prefix: Some("Bearer".to_string()),
    }),
});

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MovieImagesResponseBackdropsInner {
    #[serde(rename = "aspect_ratio", skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<f64>,
    #[serde(rename = "height", skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
    #[serde(rename = "iso_639_1", skip_serializing_if = "Option::is_none")]
    pub iso_639_1: Option<String>,
    #[serde(rename = "file_path", skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
    #[serde(rename = "vote_average", skip_serializing_if = "Option::is_none")]
    pub vote_average: Option<f64>,
    #[serde(rename = "vote_count", skip_serializing_if = "Option::is_none")]
    pub vote_count: Option<i32>,
    #[serde(rename = "width", skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
}

impl MovieImagesResponseBackdropsInner {
    pub fn new() -> Self {
        Self {
            aspect_ratio: None,
            height: None,
            iso_639_1: None,
            file_path: None,
            vote_average: None,
            vote_count: None,
            width: None,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MovieImagesResponse {
    #[serde(rename = "backdrops", skip_serializing_if = "Option::is_none")]
    pub backdrops: Option<Vec<MovieImagesResponseBackdropsInner>>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "logos", skip_serializing_if = "Option::is_none")]
    pub logos: Option<Vec<models::MovieImages200ResponseLogosInner>>,
    #[serde(rename = "posters", skip_serializing_if = "Option::is_none")]
    pub posters: Option<Vec<models::MovieImages200ResponsePostersInner>>,
}

impl MovieImagesResponse {
    pub fn new() -> Self {
        Self {
            backdrops: None,
            id: None,
            logos: None,
            posters: None,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MovieDetailsResponse {
    #[serde(rename = "adult", skip_serializing_if = "Option::is_none")]
    pub adult: Option<bool>,
    #[serde(rename = "backdrop_path", skip_serializing_if = "Option::is_none")]
    pub backdrop_path: Option<String>,
    #[serde(
        rename = "belongs_to_collection",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub belongs_to_collection: Option<Option<serde_json::Value>>,
    #[serde(rename = "budget", skip_serializing_if = "Option::is_none")]
    pub budget: Option<i32>,
    #[serde(rename = "genres", skip_serializing_if = "Option::is_none")]
    pub genres: Option<Vec<models::MovieDetails200ResponseGenresInner>>,
    #[serde(rename = "homepage", skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "imdb_id", skip_serializing_if = "Option::is_none")]
    pub imdb_id: Option<String>,
    #[serde(rename = "original_language", skip_serializing_if = "Option::is_none")]
    pub original_language: Option<String>,
    #[serde(rename = "original_title", skip_serializing_if = "Option::is_none")]
    pub original_title: Option<String>,
    #[serde(rename = "overview", skip_serializing_if = "Option::is_none")]
    pub overview: Option<String>,
    #[serde(rename = "popularity", skip_serializing_if = "Option::is_none")]
    pub popularity: Option<f64>,
    #[serde(rename = "poster_path", skip_serializing_if = "Option::is_none")]
    pub poster_path: Option<String>,
    #[serde(
        rename = "production_companies",
        skip_serializing_if = "Option::is_none"
    )]
    pub production_companies: Option<Vec<models::MovieDetails200ResponseProductionCompaniesInner>>,
    #[serde(
        rename = "production_countries",
        skip_serializing_if = "Option::is_none"
    )]
    pub production_countries: Option<Vec<models::MovieDetails200ResponseProductionCountriesInner>>,
    #[serde(rename = "release_date", skip_serializing_if = "Option::is_none")]
    pub release_date: Option<String>,
    #[serde(rename = "revenue", skip_serializing_if = "Option::is_none")]
    pub revenue: Option<i32>,
    #[serde(rename = "runtime", skip_serializing_if = "Option::is_none")]
    pub runtime: Option<i32>,
    #[serde(rename = "spoken_languages", skip_serializing_if = "Option::is_none")]
    pub spoken_languages: Option<Vec<models::MovieDetails200ResponseSpokenLanguagesInner>>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "tagline", skip_serializing_if = "Option::is_none")]
    pub tagline: Option<String>,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "video", skip_serializing_if = "Option::is_none")]
    pub video: Option<bool>,
    #[serde(rename = "vote_average", skip_serializing_if = "Option::is_none")]
    pub vote_average: Option<f64>,
    #[serde(rename = "vote_count", skip_serializing_if = "Option::is_none")]
    pub vote_count: Option<i32>,
    pub images: Option<MovieImagesResponse>,
}

pub async fn movie_details(
    configuration: &configuration::Configuration,
    movie_id: i32,
    append_to_response: Option<&str>,
    language: Option<&str>,
) -> Result<MovieDetailsResponse, Err<MovieDetailsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_movie_id = movie_id;
    let p_append_to_response = append_to_response;
    let p_language = language;

    let uri_str = format!(
        "{}/3/movie/{movie_id}",
        configuration.base_path,
        movie_id = p_movie_id
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_append_to_response {
        req_builder = req_builder.query(&[("append_to_response", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_language {
        req_builder = req_builder.query(&[("language", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{prefix} {key}"),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Err::from),
            ContentType::Text => Err(Err::from(serde_json::Error::custom(
                "Received `text/plain` content type response that cannot be converted to `models::MovieDetails200Response`",
            ))),
            ContentType::Unsupported(unknown_type) => {
                Err(Err::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::MovieDetails200Response`"
                ))))
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<MovieDetailsError> = serde_json::from_str(&content).ok();
        Err(Err::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn tv_series_details(
    configuration: &configuration::Configuration,
    series_id: i32,
    append_to_response: Option<&str>,
    language: Option<&str>,
) -> Result<TvDetailsResponse, Err<TvSeriesDetailsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_series_id = series_id;
    let p_append_to_response = append_to_response;
    let p_language = language;

    let uri_str = format!(
        "{}/3/tv/{series_id}",
        configuration.base_path,
        series_id = p_series_id
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_append_to_response {
        req_builder = req_builder.query(&[("append_to_response", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_language {
        req_builder = req_builder.query(&[("language", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{prefix} {key}"),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Err::from),
            ContentType::Text => Err(Err::from(serde_json::Error::custom(
                "Received `text/plain` content type response that cannot be converted to `models::TvSeriesDetails200Response`",
            ))),
            ContentType::Unsupported(unknown_type) => {
                Err(Err::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::TvSeriesDetails200Response`"
                ))))
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<TvSeriesDetailsError> = serde_json::from_str(&content).ok();
        Err(Err::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TvDetailsResponse {
    #[serde(rename = "adult", skip_serializing_if = "Option::is_none")]
    pub adult: Option<bool>,
    #[serde(rename = "backdrop_path", skip_serializing_if = "Option::is_none")]
    pub backdrop_path: Option<String>,
    #[serde(rename = "created_by", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<Vec<models::TvSeriesDetails200ResponseCreatedByInner>>,
    #[serde(rename = "episode_run_time", skip_serializing_if = "Option::is_none")]
    pub episode_run_time: Option<Vec<i32>>,
    #[serde(rename = "first_air_date", skip_serializing_if = "Option::is_none")]
    pub first_air_date: Option<String>,
    #[serde(rename = "genres", skip_serializing_if = "Option::is_none")]
    pub genres: Option<Vec<models::TvSeriesDetails200ResponseGenresInner>>,
    #[serde(rename = "homepage", skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "in_production", skip_serializing_if = "Option::is_none")]
    pub in_production: Option<bool>,
    #[serde(rename = "languages", skip_serializing_if = "Option::is_none")]
    pub languages: Option<Vec<String>>,
    #[serde(rename = "last_air_date", skip_serializing_if = "Option::is_none")]
    pub last_air_date: Option<String>,
    #[serde(
        rename = "last_episode_to_air",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_episode_to_air: Option<Box<models::TvSeriesDetails200ResponseLastEpisodeToAir>>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(
        rename = "next_episode_to_air",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub next_episode_to_air: Option<Option<serde_json::Value>>,
    #[serde(rename = "networks", skip_serializing_if = "Option::is_none")]
    pub networks: Option<Vec<models::TvSeriesDetails200ResponseNetworksInner>>,
    #[serde(rename = "number_of_episodes", skip_serializing_if = "Option::is_none")]
    pub number_of_episodes: Option<i32>,
    #[serde(rename = "number_of_seasons", skip_serializing_if = "Option::is_none")]
    pub number_of_seasons: Option<i32>,
    #[serde(rename = "origin_country", skip_serializing_if = "Option::is_none")]
    pub origin_country: Option<Vec<String>>,
    #[serde(rename = "original_language", skip_serializing_if = "Option::is_none")]
    pub original_language: Option<String>,
    #[serde(rename = "original_name", skip_serializing_if = "Option::is_none")]
    pub original_name: Option<String>,
    #[serde(rename = "overview", skip_serializing_if = "Option::is_none")]
    pub overview: Option<String>,
    #[serde(rename = "popularity", skip_serializing_if = "Option::is_none")]
    pub popularity: Option<f64>,
    #[serde(rename = "poster_path", skip_serializing_if = "Option::is_none")]
    pub poster_path: Option<String>,
    #[serde(
        rename = "production_companies",
        skip_serializing_if = "Option::is_none"
    )]
    pub production_companies:
        Option<Vec<models::TvSeriesDetails200ResponseProductionCompaniesInner>>,
    #[serde(
        rename = "production_countries",
        skip_serializing_if = "Option::is_none"
    )]
    pub production_countries:
        Option<Vec<models::TvSeriesDetails200ResponseProductionCountriesInner>>,
    #[serde(rename = "seasons", skip_serializing_if = "Option::is_none")]
    pub seasons: Option<Vec<TvDetailsResponseSeasonsInner>>,
    #[serde(rename = "spoken_languages", skip_serializing_if = "Option::is_none")]
    pub spoken_languages: Option<Vec<models::MovieDetails200ResponseSpokenLanguagesInner>>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "tagline", skip_serializing_if = "Option::is_none")]
    pub tagline: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "vote_average", skip_serializing_if = "Option::is_none")]
    pub vote_average: Option<f64>,
    #[serde(rename = "vote_count", skip_serializing_if = "Option::is_none")]
    pub vote_count: Option<i32>,
    pub images: Option<MovieImagesResponse>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TvDetailsResponseSeasonsInner {
    #[serde(rename = "air_date", skip_serializing_if = "Option::is_none")]
    pub air_date: Option<String>,
    #[serde(rename = "episode_count", skip_serializing_if = "Option::is_none")]
    pub episode_count: Option<i32>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "overview", skip_serializing_if = "Option::is_none")]
    pub overview: Option<String>,
    #[serde(rename = "poster_path", skip_serializing_if = "Option::is_none")]
    pub poster_path: Option<String>,
    #[serde(rename = "season_number", skip_serializing_if = "Option::is_none")]
    pub season_number: Option<i32>,
    #[serde(rename = "vote_average", skip_serializing_if = "Option::is_none")]
    pub vote_average: Option<f32>,
}

impl TvDetailsResponseSeasonsInner {
    pub fn new() -> Self {
        Self {
            air_date: None,
            episode_count: None,
            id: None,
            name: None,
            overview: None,
            poster_path: None,
            season_number: None,
            vote_average: None,
        }
    }
}
