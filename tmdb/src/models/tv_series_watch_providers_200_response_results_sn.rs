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
pub struct TvSeriesWatchProviders200ResponseResultsSn {
    #[serde(rename = "link", skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    #[serde(rename = "flatrate", skip_serializing_if = "Option::is_none")]
    pub flatrate: Option<Vec<models::TvSeriesWatchProviders200ResponseResultsGhFlatrateInner>>,
}

impl TvSeriesWatchProviders200ResponseResultsSn {
    pub fn new() -> TvSeriesWatchProviders200ResponseResultsSn {
        TvSeriesWatchProviders200ResponseResultsSn {
            link: None,
            flatrate: None,
        }
    }
}

