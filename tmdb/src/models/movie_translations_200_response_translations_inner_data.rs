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
pub struct MovieTranslations200ResponseTranslationsInnerData {
    #[serde(rename = "homepage", skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,
    #[serde(rename = "overview", skip_serializing_if = "Option::is_none")]
    pub overview: Option<String>,
    #[serde(rename = "runtime", skip_serializing_if = "Option::is_none")]
    pub runtime: Option<i32>,
    #[serde(rename = "tagline", skip_serializing_if = "Option::is_none")]
    pub tagline: Option<String>,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl MovieTranslations200ResponseTranslationsInnerData {
    pub fn new() -> MovieTranslations200ResponseTranslationsInnerData {
        MovieTranslations200ResponseTranslationsInnerData {
            homepage: None,
            overview: None,
            runtime: None,
            tagline: None,
            title: None,
        }
    }
}

