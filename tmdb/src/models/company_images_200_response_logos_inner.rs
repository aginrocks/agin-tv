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
pub struct CompanyImages200ResponseLogosInner {
    #[serde(rename = "aspect_ratio", skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<f64>,
    #[serde(rename = "file_path", skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
    #[serde(rename = "height", skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "file_type", skip_serializing_if = "Option::is_none")]
    pub file_type: Option<String>,
    #[serde(rename = "vote_average", skip_serializing_if = "Option::is_none")]
    pub vote_average: Option<f64>,
    #[serde(rename = "vote_count", skip_serializing_if = "Option::is_none")]
    pub vote_count: Option<i32>,
    #[serde(rename = "width", skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
}

impl CompanyImages200ResponseLogosInner {
    pub fn new() -> CompanyImages200ResponseLogosInner {
        CompanyImages200ResponseLogosInner {
            aspect_ratio: None,
            file_path: None,
            height: None,
            id: None,
            file_type: None,
            vote_average: None,
            vote_count: None,
            width: None,
        }
    }
}

