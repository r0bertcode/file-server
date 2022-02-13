use serde::{Deserialize, Serialize};
use wither::bson::{doc, oid::ObjectId};
use wither::Model;

/**
 * ____________________________________________________________________________________________
 * Asset data model
 * ____________________________________________________________________________________________
 * id: MongoDB ObjectId,
 * path: Path to this asset on hard disk,
 * tag: Tag of this asset to search or identify it
 * timestamp: When this Asset was creaed (stored in the DB)
 * timestamp_readable: Human readable timestamp of when created
 * ____________________________________________________________________________________________
 */
#[derive(Debug, Model, Serialize, Deserialize)]
pub struct Asset {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    #[model(index(index_type = "dsc", unique = "true"))]
    pub path: String,
    #[model(index(index_type = "dsc", unique = "true"))]
    pub uuid: String,
    pub tag: String,
    pub timestamp: String,
    pub timestamp_readable: String,
}
