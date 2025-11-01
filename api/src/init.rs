use crate::settings::Settings;
use color_eyre::eyre::Result;
use mongodb::{Client, Database};

pub async fn init_database(settings: &Settings) -> Result<Database> {
    let client = Client::with_uri_str(&settings.db.connection_string).await?;
    let database = client.database(&settings.db.database_name);

    Ok(database)
}
