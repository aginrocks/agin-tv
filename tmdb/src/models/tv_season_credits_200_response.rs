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
pub struct TvSeasonCredits200Response {
    #[serde(rename = "cast", skip_serializing_if = "Option::is_none")]
    pub cast: Option<Vec<models::TvSeriesCredits200ResponseCastInner>>,
    #[serde(rename = "crew", skip_serializing_if = "Option::is_none")]
    pub crew: Option<Vec<models::TvSeasonCredits200ResponseCrewInner>>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
}

impl TvSeasonCredits200Response {
    pub fn new() -> TvSeasonCredits200Response {
        TvSeasonCredits200Response {
            cast: None,
            crew: None,
            id: None,
        }
    }
}

