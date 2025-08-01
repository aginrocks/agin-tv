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
pub struct TvSeasonWatchProviders200ResponseResultsRu {
    #[serde(rename = "link", skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    #[serde(rename = "flatrate", skip_serializing_if = "Option::is_none")]
    pub flatrate: Option<Vec<models::TvSeriesWatchProviders200ResponseResultsRuFlatrateInner>>,
}

impl TvSeasonWatchProviders200ResponseResultsRu {
    pub fn new() -> TvSeasonWatchProviders200ResponseResultsRu {
        TvSeasonWatchProviders200ResponseResultsRu {
            link: None,
            flatrate: None,
        }
    }
}

