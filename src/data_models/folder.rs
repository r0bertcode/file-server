use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use wither::bson::{doc, oid::ObjectId};
use wither::Model;

/**
 * ____________________________________________________________________________________________
 * Folder data model
 * ____________________________________________________________________________________________
 * id: MongoDB ObjectId
 * path: Path to this folder on disk (Unique)
 * files: Vec of all ObjectIds of the files in this folder
 * is_public: Flag to represent if this folder and its assets can be accessed by anyone, including the public (Non-users)
 * access_groups: Vec of the ObjectIds of the AccessGroups of this folder ( Who can access this folder and its assets )
 * timestamp: When this folder was created
 * log_activity: A flag for if this flag should log its activity ( entries, deletions, access group changes )
 * activity_log: A log map ( if log_activity is false is None ) of events on this folder ( Can be used for debugging or logging if you need it )
 * ____________________________________________________________________________________________
 */
#[derive(Debug, Model, Serialize, Deserialize)]
pub struct Folder {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    #[model(index(index_type = "dsc", unique = "true"))]
    pub path: String,
    pub files: Vec<ObjectId>,
    pub is_public: bool,
    pub access_groups: Vec<ObjectId>,
    pub timestamp: String,
    pub log_activity: bool,
    pub activity_log: Option<BTreeMap<String, String>>,
}
