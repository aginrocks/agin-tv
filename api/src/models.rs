use crate::mongo_id::object_id_as_string_required;
use mongodb::bson::oid::ObjectId;
use partial_struct::Partial;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use visible::StructFields;

macro_rules! database_object {
    ($name:ident { $($field:tt)* }$(, $($omitfield:ident),*)?) => {
        #[derive(Partial, Debug, Serialize, Deserialize, ToSchema, Clone)]
        #[partial(omit(id $(, $($omitfield),* )?), derive(Debug, Serialize, Deserialize, ToSchema, Clone))]
        #[StructFields(pub)]
        pub struct $name {
            $($field)*
        }
    };
}

database_object!(User {
    #[serde(rename = "_id", with = "object_id_as_string_required")]
    #[schema(value_type = String)]
    id: ObjectId,
    subject: String,
    name: String,
    email: String,
});

database_object!(Movie {
    #[serde(rename = "_id", with = "object_id_as_string_required")]
    #[schema(value_type = String)]
    id: ObjectId,
    name: String,
    original_name: Option<String>,
    description: String,
    release_year: i32,
    genre: Vec<String>,
});
