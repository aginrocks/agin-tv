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
pub struct TvSeriesChanges200ResponseChangesInnerItemsInnerOriginalValue {
    #[serde(rename = "poster", skip_serializing_if = "Option::is_none")]
    pub poster: Option<Box<models::TvSeriesChanges200ResponseChangesInnerItemsInnerOriginalValuePoster>>,
}

impl TvSeriesChanges200ResponseChangesInnerItemsInnerOriginalValue {
    pub fn new() -> TvSeriesChanges200ResponseChangesInnerItemsInnerOriginalValue {
        TvSeriesChanges200ResponseChangesInnerItemsInnerOriginalValue {
            poster: None,
        }
    }
}

