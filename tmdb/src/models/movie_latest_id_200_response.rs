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
pub struct MovieLatestId200Response {
    #[serde(rename = "adult", skip_serializing_if = "Option::is_none")]
    pub adult: Option<bool>,
    #[serde(rename = "backdrop_path", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub backdrop_path: Option<Option<serde_json::Value>>,
    #[serde(rename = "belongs_to_collection", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub belongs_to_collection: Option<Option<serde_json::Value>>,
    #[serde(rename = "budget", skip_serializing_if = "Option::is_none")]
    pub budget: Option<i32>,
    #[serde(rename = "genres", skip_serializing_if = "Option::is_none")]
    pub genres: Option<Vec<serde_json::Value>>,
    #[serde(rename = "homepage", skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "imdb_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub imdb_id: Option<Option<serde_json::Value>>,
    #[serde(rename = "original_language", skip_serializing_if = "Option::is_none")]
    pub original_language: Option<String>,
    #[serde(rename = "original_title", skip_serializing_if = "Option::is_none")]
    pub original_title: Option<String>,
    #[serde(rename = "overview", skip_serializing_if = "Option::is_none")]
    pub overview: Option<String>,
    #[serde(rename = "popularity", skip_serializing_if = "Option::is_none")]
    pub popularity: Option<i32>,
    #[serde(rename = "poster_path", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub poster_path: Option<Option<serde_json::Value>>,
    #[serde(rename = "production_companies", skip_serializing_if = "Option::is_none")]
    pub production_companies: Option<Vec<serde_json::Value>>,
    #[serde(rename = "production_countries", skip_serializing_if = "Option::is_none")]
    pub production_countries: Option<Vec<serde_json::Value>>,
    #[serde(rename = "release_date", skip_serializing_if = "Option::is_none")]
    pub release_date: Option<String>,
    #[serde(rename = "revenue", skip_serializing_if = "Option::is_none")]
    pub revenue: Option<i32>,
    #[serde(rename = "runtime", skip_serializing_if = "Option::is_none")]
    pub runtime: Option<i32>,
    #[serde(rename = "spoken_languages", skip_serializing_if = "Option::is_none")]
    pub spoken_languages: Option<Vec<serde_json::Value>>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "tagline", skip_serializing_if = "Option::is_none")]
    pub tagline: Option<String>,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "video", skip_serializing_if = "Option::is_none")]
    pub video: Option<bool>,
    #[serde(rename = "vote_average", skip_serializing_if = "Option::is_none")]
    pub vote_average: Option<i32>,
    #[serde(rename = "vote_count", skip_serializing_if = "Option::is_none")]
    pub vote_count: Option<i32>,
}

impl MovieLatestId200Response {
    pub fn new() -> MovieLatestId200Response {
        MovieLatestId200Response {
            adult: None,
            backdrop_path: None,
            belongs_to_collection: None,
            budget: None,
            genres: None,
            homepage: None,
            id: None,
            imdb_id: None,
            original_language: None,
            original_title: None,
            overview: None,
            popularity: None,
            poster_path: None,
            production_companies: None,
            production_countries: None,
            release_date: None,
            revenue: None,
            runtime: None,
            spoken_languages: None,
            status: None,
            tagline: None,
            title: None,
            video: None,
            vote_average: None,
            vote_count: None,
        }
    }
}

