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
pub struct AlternativeNamesCopy200Response {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "logos", skip_serializing_if = "Option::is_none")]
    pub logos: Option<Vec<models::AlternativeNamesCopy200ResponseLogosInner>>,
}

impl AlternativeNamesCopy200Response {
    pub fn new() -> AlternativeNamesCopy200Response {
        AlternativeNamesCopy200Response {
            id: None,
            logos: None,
        }
    }
}

