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
pub struct TvSeriesWatchProviders200ResponseResultsGb {
    #[serde(rename = "link", skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    #[serde(rename = "flatrate", skip_serializing_if = "Option::is_none")]
    pub flatrate: Option<Vec<models::TvSeriesWatchProviders200ResponseResultsGbFlatrateInner>>,
    #[serde(rename = "buy", skip_serializing_if = "Option::is_none")]
    pub buy: Option<Vec<models::MovieWatchProviders200ResponseResultsGbBuyInner>>,
}

impl TvSeriesWatchProviders200ResponseResultsGb {
    pub fn new() -> TvSeriesWatchProviders200ResponseResultsGb {
        TvSeriesWatchProviders200ResponseResultsGb {
            link: None,
            flatrate: None,
            buy: None,
        }
    }
}

