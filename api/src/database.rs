use crate::mongo_id::object_id_as_string_required;
use crate::settings::Settings;

use color_eyre::eyre::Result;
use mongodb::{Client, Database, bson::oid::ObjectId};
use partial_struct::Partial;
use serde::{Deserialize, Serialize};
use tower_sessions::{
    Expiry, SessionManagerLayer,
    cookie::{SameSite, time::Duration},
};
use tower_sessions_redis_store::{
    RedisStore,
    fred::prelude::{ClientLike, Config, Pool},
};
use utoipa::ToSchema;
use visible::StructFields;

pub async fn init_database(settings: &Settings) -> Result<Database> {
    let client = Client::with_uri_str(&settings.db.connection_string).await?;
    let database = client.database(&settings.db.database_name);

    Ok(database)
}

pub async fn init_session_store(
    settings: &Settings,
) -> Result<SessionManagerLayer<RedisStore<Pool>>> {
    let config = Config::from_url(&settings.redis.connection_string)?;
    let pool = Pool::new(config, None, None, None, 6)?;

    let _redis_conn = pool.connect();
    pool.wait_for_connect().await?;

    let session_store = RedisStore::<Pool>::new(pool);

    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_same_site(SameSite::Lax)
        .with_expiry(Expiry::OnInactivity(Duration::days(7)));

    Ok(session_layer)
}

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
