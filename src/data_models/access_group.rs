use serde::{Deserialize, Serialize};
use wither::bson::{doc, oid::ObjectId};
use wither::Model;
/**
 * ____________________________________________________________________________________________
 * AccessGroup data model
 * ____________________________________________________________________________________________
 * id: MongoDB ObjectId,
 * tag: Tag to name this access group
 * allowed_keys: List of kets in this access_group, any user will be able to access this folder if they have a key in this list
 * timestamp: When the access group was created
 * ____________________________________________________________________________________________
 */
#[derive(Debug, Model, Serialize, Deserialize)]
pub struct AccessGroup {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    #[model(index(index_type = "dsc", unique = "true"))]
    pub tag: String,
    pub allowed_keys: Vec<ObjectId>,
    pub timestamp: String,
}
