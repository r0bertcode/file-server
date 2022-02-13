mod constants;
mod controller;
mod data_models;
mod util;

use constants::{ASSET_MAIN_PATH, DB_NAME, MONGO_URI};
use data_models::{access_group::AccessGroup, asset::Asset, folder::Folder, key::Key, user::User};
use std::fs::create_dir_all;
use std::path::Path;
use wither::mongodb::Client;
use wither::{prelude::*, Result};

use controller::file_system::create_folder;

#[tokio::main]
async fn main() -> Result<()> {
    // Create asset directory if not present
    if !Path::new(ASSET_MAIN_PATH).exists() {
        create_dir_all(ASSET_MAIN_PATH).expect(&format!(
            "Fatal: Could not create required directory {}",
            ASSET_MAIN_PATH
        ));
    }

    // Connect to MongoDB and sync indexes on all Models
    let db = Client::with_uri_str(MONGO_URI).await?.database(DB_NAME);
    Key::sync(&db).await?;
    User::sync(&db).await?;
    Asset::sync(&db).await?;
    Folder::sync(&db).await?;
    AccessGroup::sync(&db).await?;

    let id = create_folder(&db, "first-folder", None).await.unwrap();
    println!("{}", id);

    Ok(())
}
