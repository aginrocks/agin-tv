/*
 * TMDB API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TvSeriesDetails200Response {
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
    #[serde(rename = "last_episode_to_air", skip_serializing_if = "Option::is_none")]
    pub last_episode_to_air: Option<Box<models::TvSeriesDetails200ResponseLastEpisodeToAir>>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "next_episode_to_air", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
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
    #[serde(rename = "production_companies", skip_serializing_if = "Option::is_none")]
    pub production_companies: Option<Vec<models::TvSeriesDetails200ResponseProductionCompaniesInner>>,
    #[serde(rename = "production_countries", skip_serializing_if = "Option::is_none")]
    pub production_countries: Option<Vec<models::TvSeriesDetails200ResponseProductionCountriesInner>>,
    #[serde(rename = "seasons", skip_serializing_if = "Option::is_none")]
    pub seasons: Option<Vec<models::TvSeriesDetails200ResponseSeasonsInner>>,
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
}

impl TvSeriesDetails200Response {
    pub fn new() -> TvSeriesDetails200Response {
        TvSeriesDetails200Response {
            adult: None,
            backdrop_path: None,
            created_by: None,
            episode_run_time: None,
            first_air_date: None,
            genres: None,
            homepage: None,
            id: None,
            in_production: None,
            languages: None,
            last_air_date: None,
            last_episode_to_air: None,
            name: None,
            next_episode_to_air: None,
            networks: None,
            number_of_episodes: None,
            number_of_seasons: None,
            origin_country: None,
            original_language: None,
            original_name: None,
            overview: None,
            popularity: None,
            poster_path: None,
            production_companies: None,
            production_countries: None,
            seasons: None,
            spoken_languages: None,
            status: None,
            tagline: None,
            r#type: None,
            vote_average: None,
            vote_count: None,
        }
    }
}

