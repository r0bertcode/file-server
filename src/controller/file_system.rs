use crate::constants::ASSET_MAIN_PATH;
use crate::controller::error::ControllerError;
use crate::data_models::{
    access_group::AccessGroup, asset::Asset, folder::Folder, key::Key, user::User,
};
use crate::util::{get_readable_timestamp, get_timestamp};
use std::fs::*;
use std::path::Path;
use wither::{
    mongodb::{
        bson::{doc, oid::ObjectId},
        Database,
    },
    Model,
};

/**
 * Controller to create folder and associated DB operations
 */
pub async fn create_folder(
    db_ref: &Database,
    folder_name: &str,
    start_access_group: Option<ObjectId>,
) -> Result<ObjectId, ControllerError> {
    // Format path to folder
    let folder_path = format!("{}/{}", ASSET_MAIN_PATH, folder_name);
    // Check if a folder with this name already exists
    if Path::new(&folder_path).exists() {
        return Err(ControllerError {
        io: None,
        wither: None,
        operation: Some("Cannot create a folder that already exists, use another name or delete the other folder!".to_string())
      });
    }

    // Attempt to create the directory on disk
    let create_dir_result = create_dir_all(&folder_path);
    if create_dir_result.is_err() {
        return Err(ControllerError {
            io: create_dir_result.err(),
            wither: None,
            operation: None,
        });
    }

    let mut folder_doc;
    // Get meta data for folder Doc
    let timestamp_num = get_timestamp();
    let timestamp = timestamp_num.to_string();
    let timestamp_readable = get_readable_timestamp(timestamp_num);

    if start_access_group.is_none() {
        // If a starting access_group ObjectId is not provided, the folder will start as public
        folder_doc = Folder {
            id: None,
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
            operation: None,
        });
    }

    let doc_id = folder_doc.id();
    if doc_id.is_none() {
        return Err(ControllerError {
        io: None,
        wither: None,
        operation: Some("FATAL: Was unable to get folder docs _id field after saving in MongoDB succesfully..".to_string())
      });
    }

    // Return ObjectId if all went well
    Ok(doc_id.unwrap())
}

// pub async fn save_asset(
//     db_ref: &Database,
//     file_data: &[u8],
//     folder_name: &str,
//     extension: &str,
// ) -> Result<(), ControllerError> {
//     // format folder path and make sure it exists
//     let folder_path = format!("{}/{}", ASSET_MAIN_PATH, folder_name);
//     if !Path::new(&folder_path).exists() {
//         return Err(ControllerError {
//         io: None,
//         wither: None,
//         operation: Some(format!("ERROR: Bad operation, attempted to save asset at path {} but this folder doesn't exist!", folder_path))
//       });
//     }

//     let asset_path = format!("{}/{}.{}", folder_path, )
//     Ok(())
// }
