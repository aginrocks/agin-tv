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
pub struct CertificationsTvList200Response {
    #[serde(rename = "certifications", skip_serializing_if = "Option::is_none")]
    pub certifications: Option<Box<models::CertificationsTvList200ResponseCertifications>>,
}

impl CertificationsTvList200Response {
    pub fn new() -> CertificationsTvList200Response {
        CertificationsTvList200Response {
            certifications: None,
        }
    }
}

