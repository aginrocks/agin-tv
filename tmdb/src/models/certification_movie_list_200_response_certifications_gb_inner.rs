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
pub struct CertificationMovieList200ResponseCertificationsGbInner {
    #[serde(rename = "certification", skip_serializing_if = "Option::is_none")]
    pub certification: Option<String>,
    #[serde(rename = "meaning", skip_serializing_if = "Option::is_none")]
    pub meaning: Option<String>,
    #[serde(rename = "order", skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
}

impl CertificationMovieList200ResponseCertificationsGbInner {
    pub fn new() -> CertificationMovieList200ResponseCertificationsGbInner {
        CertificationMovieList200ResponseCertificationsGbInner {
            certification: None,
            meaning: None,
            order: None,
        }
    }
}

