use serde::{Deserialize, Serialize};
use wither::bson::{doc, oid::ObjectId};
use wither::Model;
/**
 * ____________________________________________________________________________________________
 * Key data model
 * ____________________________________________________________________________________________
 * id: MongoDB ObjectId
 * uuid: Unique UUID v4 for identity
 * active: Bool for weither this key can be used or not ( If it is active or not )
 * timestamp: When this key was created
 * ____________________________________________________________________________________________
 */
#[derive(Debug, Model, Serialize, Deserialize)]
pub struct Key {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    #[model(index(index_type = "dsc", unique = "true"))]
    pub uuid: String,
    pub active: bool,
    pub timestamp: String,
}
