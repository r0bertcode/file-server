use serde::{Deserialize, Serialize};
use wither::bson::{doc, oid::ObjectId};
use wither::Model;

/**
 * ____________________________________________________________________________________________
 * Asset data model
 * ____________________________________________________________________________________________
 * id: MongoDB ObjectId,
 * path: Path to this asset on hard disk,
 * tags: Vec of tag strings to group this key with others or identify it by readable words
 * timestamp: When this Asset was creaed (stored in the DB)
 * ____________________________________________________________________________________________
 */
#[derive(Debug, Model, Serialize, Deserialize)]
pub struct Asset {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    #[model(index(index_type = "dsc", unique = "true"))]
    pub path: String,
    pub tags: Vec<String>,
    pub timestamp: String,
}
