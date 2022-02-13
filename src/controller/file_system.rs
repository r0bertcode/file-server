use crate::constants::ASSET_MAIN_PATH;
use crate::controller::error::ControllerError;
use crate::data_models::{
    access_group::AccessGroup, asset::Asset, folder::Folder, key::Key, user::User,
};
use crate::util::{get_time_meta, get_uuid};
use bcrypt::{hash, DEFAULT_COST};
use std::fs::*;
use std::path::Path;
use wither::{
    mongodb::{
        bson::{doc, oid::ObjectId},
        Database,
    },
    prelude::*,
    Model,
};
/**
 * Controller to create a user for the file system
 */
pub async fn create_user(
    db_ref: &Database,
    user: &str,
    pass: &str,
) -> Result<User, ControllerError> {
    // Attempt to hash pass via bcrypt
    let hashed_pass = hash(pass, DEFAULT_COST + 1);
    if hashed_pass.is_err() {
        return Err(ControllerError {
            io: None,
            wither: None,
            bcrypt: hashed_pass.err(),
            operation: None,
        });
    }

    let (timestamp, timestamp_readable) = get_time_meta();
    let mut user_doc = User {
        id: None,
        user: user.to_string(),
        pass: hashed_pass.unwrap(),
        keys: vec![],
        key_admins: vec![],
        user_admins: vec![],
        folder_admins: vec![],
        access_group_admins: vec![],
        timestamp,
        timestamp_readable,
    };

    // Attempt to save folder doc
    let save_result = user_doc.save(db_ref, None).await;
    if save_result.is_err() {
        return Err(ControllerError {
            io: None,
            wither: save_result.err(),
            bcrypt: None,
            operation: None,
        });
    }

    // Attempt to get ObjectId from saved doc
    let doc_id = user_doc.id();
    if doc_id.is_none() {
        return Err(ControllerError {
            io: None,
            wither: None,
            bcrypt: None,
            operation: Some("FATAL: Was unable to get folder docs _id field after saving in MongoDB succesfully..".to_string())
          });
    }

    Ok(user_doc)
}
/**
 * Controller to create folder and associated DB operations
 */
pub async fn create_folder(
    db_ref: &Database,
    admin: &ObjectId,
    folder_name: &str,
    start_access_group: Option<ObjectId>,
    static_folder_path: Option<&str>,
) -> Result<Folder, ControllerError> {
    let folder_path;

    if static_folder_path.is_some() {
        folder_path = static_folder_path.unwrap().to_string();
    } else {
        folder_path = format!("{}/{}", ASSET_MAIN_PATH, folder_name);
    }

    // Check if a folder with this name already exists
    if Path::new(&folder_path).exists() {
        return Err(ControllerError {
        io: None,
        wither: None,
        bcrypt: None,
        operation: Some("Cannot create a folder that already exists, use another name or delete the other folder!".to_string())
      });
    }

    // Attempt to create the directory on disk
    let create_dir_result = create_dir_all(&folder_path);
    if create_dir_result.is_err() {
        return Err(ControllerError {
            io: create_dir_result.err(),
            wither: None,
            bcrypt: None,
            operation: None,
        });
    }

    let mut folder_doc;
    // Get meta data for folder Doc
    let (timestamp, timestamp_readable) = get_time_meta();

    if start_access_group.is_none() {
        // If a starting access_group ObjectId is not provided, the folder will start as public
        folder_doc = Folder {
            id: None,
            tag: folder_name.to_string(),
            path: folder_path,
            files: vec![],
            is_public: true,
            access_groups: vec![],
            timestamp,
            timestamp_readable,
        }
    } else {
        // Otherwise, it will be private and bound to the starting access_group
        folder_doc = Folder {
            id: None,
            tag: folder_name.to_string(),
            path: folder_path,
            files: vec![],
            is_public: false,
            access_groups: vec![start_access_group.unwrap()],
            timestamp,
            timestamp_readable,
        }
    }

    // Attempt to save folder doc
    let save_result = folder_doc.save(db_ref, None).await;
    if save_result.is_err() {
        return Err(ControllerError {
            io: None,
            wither: save_result.err(),
            bcrypt: None,
            operation: None,
        });
    }

    // Attempt to get ObjectId
    let doc_id = folder_doc.id();
    if doc_id.is_none() {
        return Err(ControllerError {
        io: None,
        wither: None,
        bcrypt: None,
        operation: Some("ERROR: Was unable to get folder docs _id field after saving in MongoDB succesfully..".to_string())
      });
    }

    let _id = doc_id.unwrap();

    // Attempt to add this folders _id to the admins admin folder list
    let user_doc_result = User::find_one_and_update(
        db_ref,
        doc! { "_id": &admin },
        doc! { "$push": doc! { "folder_admins": &_id } },
        None,
    )
    .await;

    if user_doc_result.is_err() {
        return Err(ControllerError {
            io: None,
            wither: user_doc_result.err(),
            bcrypt: None,
            operation: None,
        });
    }

    // Verify a user exist and was updated, otherwise this folder has no admin
    let user_doc = user_doc_result.unwrap();
    if user_doc.is_none() {
        // We need to clear this folder as it has no admin
        let folder_remove_result = folder_doc.delete(db_ref).await;
        if folder_remove_result.is_err() {
            return Err(ControllerError {
                io: None,
                wither: folder_remove_result.err(),
                bcrypt: None,
                operation: None,
            });
        }

        return Err(ControllerError {
            io: None,
            wither: None,
            bcrypt: None,
            operation: Some(format!("ERROR: Was unable to find a user by the ObjectId {}, a folder needs to have a admin to be created!", admin)),
        });
    }

    // Return ObjectId if all went well
    Ok(folder_doc)
}
/**
 * Controller to create a sub folder
 */
pub async fn create_sub_folder(
    db_ref: &Database,
    admin: &ObjectId,
    parent_path: &str,
    folder_name: &str,
    start_access_group: Option<ObjectId>,
) -> Result<Folder, ControllerError> {
    let sub_path = format!("{}/{}", parent_path, folder_name);
    let folder = create_folder(
        db_ref,
        admin,
        folder_name,
        start_access_group,
        Some(&sub_path),
    )
    .await?;

    Ok(folder)
}
/**
 * Controller to save an asset(file) on disk and associated DB data
 */
pub async fn save_asset(
    db_ref: &Database,
    file_data: Vec<u8>,
    tag: &str,
    folder_path: &str,
    extension: &str,
) -> Result<ObjectId, ControllerError> {
    // format folder path and make sure it exists
    if !Path::new(&folder_path).exists() {
        return Err(ControllerError {
        io: None,
        wither: None,
        bcrypt: None,
        operation: Some(format!("ERROR: Bad operation, attempted to save asset at path {} but this folder doesn't exist!", folder_path))
      });
    }

    // Generate uuid and asset_path and asset in the very off chance, one with this uuid doesn't already exist there
    let uuid = get_uuid();
    let asset_path = format!("{}/{}.{}", folder_path, uuid, extension);
    if Path::new(&asset_path).exists() {
        return Err(ControllerError {
        io: None,
        wither: None,
        bcrypt: None,
        operation: Some(format!("ERROR: Bad operation, attempted to save asset at path {}, but this asset already exists here!", asset_path))
      });
    }

    // Get meta data for asset Doc
    let (timestamp, timestamp_readable) = get_time_meta();

    let mut asset_doc = Asset {
        id: None,
        uuid,
        tag: tag.to_string(),
        path: asset_path.clone(),
        timestamp,
        timestamp_readable,
    };

    // Attempt to save asset doc
    let save_result = asset_doc.save(db_ref, None).await;
    if save_result.is_err() {
        return Err(ControllerError {
            io: None,
            wither: save_result.err(),
            bcrypt: None,
            operation: None,
        });
    }

    // Attempt to get _id after save
    let doc_id = asset_doc.id();
    if doc_id.is_none() {
        return Err(ControllerError {
        io: None,
        wither: None,
        bcrypt: None,
        operation: Some("FATAL: Was unable to get folder docs _id field after saving in MongoDB succesfully..".to_string())
      });
    }

    // Attempt to write data to disk to path
    let write_result = write(asset_path, file_data);
    if write_result.is_err() {
        return Err(ControllerError {
            io: write_result.err(),
            wither: None,
            bcrypt: None,
            operation: None,
        });
    }

    // Return ObjectId if all goes well
    Ok(doc_id.unwrap())
}
