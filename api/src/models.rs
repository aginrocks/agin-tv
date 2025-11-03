pub mod movie;

use bson::Document;
use movie::TMDBMovieId;

use crate::mongo_id::{object_id_as_string_required, vec_oid_to_vec_string};
use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use partial_struct::Partial;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use visible::StructFields;

macro_rules! database_object {
    ($name:ident { $($field:tt)* }$(, $($omitfield:ident),*)?) => {
        #[derive(Partial, Debug, Serialize, Deserialize, ToSchema, Clone, Default)]
        #[partial(omit(id $(, $($omitfield),* )?), derive(Debug, Serialize, Deserialize, ToSchema, Clone))]
        #[StructFields(pub)]
        pub struct $name {
            $($field)*

        }
    };
}

database_object!(Token {
    #[serde(rename = "_id", with = "object_id_as_string_required")]
    #[schema(value_type = String)]
    id: ObjectId,
    #[serde(with = "object_id_as_string_required")]
    #[schema(value_type = String)]
    user_id: ObjectId,
    token: String,
});

database_object!(User {
    #[serde(rename = "_id", with = "object_id_as_string_required")]
    #[schema(value_type = String)]
    id: ObjectId,
    name: String,
    email: String,
    sub: String,
});

database_object!(
    Movie {
    #[serde(rename = "_id", with = "object_id_as_string_required")]
    #[schema(value_type = String)]
    id: ObjectId,
    #[schema(value_type = String)]
    tmdb_id: TMDBMovieId,
    name: String,
    original_name: Option<String>,
    description: String,
    tv: bool,
    #[schema(value_type = String)]
    release_date: Option<DateTime<Utc>>,
    vertical_cover_url: Option<String>,
    horizontal_cover_url: Option<String>,
    background_url: Option<String>,
    logo_url: Option<String>,
    #[serde(with = "vec_oid_to_vec_string")]
    #[schema(value_type = Vec<Genre>)]
    genres: Vec<ObjectId>,
    original_language: Option<String>,
});

database_object!(Genre {
    #[serde(rename = "_id", with = "object_id_as_string_required")]
    #[schema(value_type = String)]
    id: ObjectId,
    tmdb_id: u32,
    name: String,
});

impl From<Genre> for Document {
    fn from(val: Genre) -> Self {
        let mut doc = bson::to_document(&val).expect("Failed to convert Movie to Document");
        doc.remove("_id");
        doc
    }
}
