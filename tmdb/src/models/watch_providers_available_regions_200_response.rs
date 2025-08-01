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
pub struct WatchProvidersAvailableRegions200Response {
    #[serde(rename = "results", skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<models::WatchProvidersAvailableRegions200ResponseResultsInner>>,
}

impl WatchProvidersAvailableRegions200Response {
    pub fn new() -> WatchProvidersAvailableRegions200Response {
        WatchProvidersAvailableRegions200Response {
            results: None,
        }
    }
}

