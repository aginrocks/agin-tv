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
pub struct TvSeriesWatchProviders200ResponseResultsRu {
    #[serde(rename = "link", skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    #[serde(rename = "flatrate", skip_serializing_if = "Option::is_none")]
    pub flatrate: Option<Vec<models::TvSeriesWatchProviders200ResponseResultsRuFlatrateInner>>,
    #[serde(rename = "ads", skip_serializing_if = "Option::is_none")]
    pub ads: Option<Vec<models::TvSeriesWatchProviders200ResponseResultsRuAdsInner>>,
}

impl TvSeriesWatchProviders200ResponseResultsRu {
    pub fn new() -> TvSeriesWatchProviders200ResponseResultsRu {
        TvSeriesWatchProviders200ResponseResultsRu {
            link: None,
            flatrate: None,
            ads: None,
        }
    }
}

