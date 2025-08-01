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
pub struct TvEpisodeTranslations200ResponseTranslationsInner {
    #[serde(rename = "iso_3166_1", skip_serializing_if = "Option::is_none")]
    pub iso_3166_1: Option<String>,
    #[serde(rename = "iso_639_1", skip_serializing_if = "Option::is_none")]
    pub iso_639_1: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "english_name", skip_serializing_if = "Option::is_none")]
    pub english_name: Option<String>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Box<models::TvEpisodeTranslations200ResponseTranslationsInnerData>>,
}

impl TvEpisodeTranslations200ResponseTranslationsInner {
    pub fn new() -> TvEpisodeTranslations200ResponseTranslationsInner {
        TvEpisodeTranslations200ResponseTranslationsInner {
            iso_3166_1: None,
            iso_639_1: None,
            name: None,
            english_name: None,
            data: None,
        }
    }
}

