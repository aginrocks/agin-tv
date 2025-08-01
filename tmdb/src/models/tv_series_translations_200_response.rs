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
pub struct TvSeriesTranslations200Response {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "translations", skip_serializing_if = "Option::is_none")]
    pub translations: Option<Vec<models::TvSeriesTranslations200ResponseTranslationsInner>>,
}

impl TvSeriesTranslations200Response {
    pub fn new() -> TvSeriesTranslations200Response {
        TvSeriesTranslations200Response {
            id: None,
            translations: None,
        }
    }
}

