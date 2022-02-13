use serde::{Deserialize, Serialize};
use wither::bson::{doc, oid::ObjectId};
use wither::Model;

/**
 * ____________________________________________________________________________________________
 * User data model
 * ____________________________________________________________________________________________
 * id: MongoDB ObjectId
 * user: Username (Unique)
 * pass: Password (Bcrypt hashed version is stored in DB)
 * keys: Vec of ObjectIds of keys this user has access to use
 * key_admins: Vec of ObjectIds of keys this user has access to delete/edit
 * user_admins: Vec of ObjectIds of users this user has access to delete/edit
 * folder_admins: Vec of ObjectIds of folders this user has access to delete/edit
 * access_group_admins: Vec of ObjectIds of acces groups this user has access to delete/edit
 * timestamp: When this user was created
 * timestamp_readable: Human readable timestamp of when created
 * ____________________________________________________________________________________________
 */
#[derive(Debug, Model, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    #[model(index(index_type = "dsc", unique = "true"))]
    pub user: String,
    pub pass: String,
    pub keys: Vec<ObjectId>,
    pub key_admins: Vec<ObjectId>,
    pub user_admins: Vec<ObjectId>,
    pub folder_admins: Vec<ObjectId>,
    pub access_group_admins: Vec<ObjectId>,
    pub timestamp: String,
    pub timestamp_readable: String,
}
