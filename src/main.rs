mod constants;
mod controller;
mod data_models;
mod util;

use constants::{ASSET_MAIN_PATH, DB_NAME, MONGO_URI};
use data_models::{access_group::AccessGroup, asset::Asset, folder::Folder, key::Key, user::User};
use std::fs::create_dir_all;
use std::path::Path;
use wither::mongodb::Client;
use wither::{mongodb::bson::doc, prelude::*, Result};

use controller::{
    auth::login_user,
    file_system::{create_folder, create_user, save_asset},
};
use util::get_file_data;

use crate::controller::file_system::create_sub_folder;

#[tokio::main]
async fn main() -> Result<()> {
    // Create asset directory if not present
    if !Path::new(ASSET_MAIN_PATH).exists() {
        create_dir_all(ASSET_MAIN_PATH).expect(&format!(
            "ERROR: Could not create required directory {}",
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

    //===================== TEST SECTION  ========================///
    Key::delete_many(&db, doc! {}, None).await.unwrap();
    User::delete_many(&db, doc! {}, None).await.unwrap();
    Asset::delete_many(&db, doc! {}, None).await.unwrap();
    Folder::delete_many(&db, doc! {}, None).await.unwrap();
    AccessGroup::delete_many(&db, doc! {}, None).await.unwrap();

    let user_doc = create_user(&db, "admin", "root").await.unwrap();

    let user_doc = login_user(&db, "admin", "root").await.unwrap().unwrap();

    let folder = create_folder(&db, &user_doc.id.as_ref().unwrap(), "admins", None, None)
        .await
        .unwrap();

    let sub_folder = create_sub_folder(
        &db,
        &user_doc.id.as_ref().unwrap(),
        &folder.path,
        "admin-sub",
        None,
    )
    .await
    .unwrap();

    let sub_sub_folder = create_sub_folder(
        &db,
        &user_doc.id.unwrap(),
        &sub_folder.path,
        "admin-sub-sub",
        None,
    )
    .await
    .unwrap();

    let file_data = get_file_data("./test_video.mp4").unwrap();
    let asset_id = save_asset(&db, file_data, "my_video", &sub_sub_folder.path, "mp4")
        .await
        .unwrap();

    println!("{:?}", asset_id);

    Ok(())
}
