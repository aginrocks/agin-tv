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
pub struct MovieWatchProviders200ResponseResultsBe {
    #[serde(rename = "link", skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    #[serde(rename = "rent", skip_serializing_if = "Option::is_none")]
    pub rent: Option<Vec<models::MovieWatchProviders200ResponseResultsBeRentInner>>,
    #[serde(rename = "flatrate", skip_serializing_if = "Option::is_none")]
    pub flatrate: Option<Vec<models::MovieWatchProviders200ResponseResultsBeFlatrateInner>>,
    #[serde(rename = "buy", skip_serializing_if = "Option::is_none")]
    pub buy: Option<Vec<models::MovieWatchProviders200ResponseResultsBeRentInner>>,
}

impl MovieWatchProviders200ResponseResultsBe {
    pub fn new() -> MovieWatchProviders200ResponseResultsBe {
        MovieWatchProviders200ResponseResultsBe {
            link: None,
            rent: None,
            flatrate: None,
            buy: None,
        }
    }
}

