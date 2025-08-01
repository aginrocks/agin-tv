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
pub struct TvSeriesRecommendations200ResponseResultsInner {
    #[serde(rename = "adult", skip_serializing_if = "Option::is_none")]
    pub adult: Option<bool>,
    #[serde(rename = "backdrop_path", skip_serializing_if = "Option::is_none")]
    pub backdrop_path: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "original_language", skip_serializing_if = "Option::is_none")]
    pub original_language: Option<String>,
    #[serde(rename = "original_name", skip_serializing_if = "Option::is_none")]
    pub original_name: Option<String>,
    #[serde(rename = "overview", skip_serializing_if = "Option::is_none")]
    pub overview: Option<String>,
    #[serde(rename = "poster_path", skip_serializing_if = "Option::is_none")]
    pub poster_path: Option<String>,
    #[serde(rename = "media_type", skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
    #[serde(rename = "genre_ids", skip_serializing_if = "Option::is_none")]
    pub genre_ids: Option<Vec<i32>>,
    #[serde(rename = "popularity", skip_serializing_if = "Option::is_none")]
    pub popularity: Option<f64>,
    #[serde(rename = "first_air_date", skip_serializing_if = "Option::is_none")]
    pub first_air_date: Option<String>,
    #[serde(rename = "vote_average", skip_serializing_if = "Option::is_none")]
    pub vote_average: Option<f64>,
    #[serde(rename = "vote_count", skip_serializing_if = "Option::is_none")]
    pub vote_count: Option<i32>,
    #[serde(rename = "origin_country", skip_serializing_if = "Option::is_none")]
    pub origin_country: Option<Vec<String>>,
}

impl TvSeriesRecommendations200ResponseResultsInner {
    pub fn new() -> TvSeriesRecommendations200ResponseResultsInner {
        TvSeriesRecommendations200ResponseResultsInner {
            adult: None,
            backdrop_path: None,
            id: None,
            name: None,
            original_language: None,
            original_name: None,
            overview: None,
            poster_path: None,
            media_type: None,
            genre_ids: None,
            popularity: None,
            first_air_date: None,
            vote_average: None,
            vote_count: None,
            origin_country: None,
        }
    }
}

