use crate::controller::error::ControllerError;
use crate::data_models::{
    access_group::AccessGroup, asset::Asset, folder::Folder, key::Key, user::User,
};
use crate::util::{get_time_meta, get_uuid};
use bcrypt::verify;
use wither::{
    mongodb::{
        bson::{doc, oid::ObjectId},
        Database,
    },
    Model,
};

/**
 * Attempt to login user with username and unhashed password
 */
pub async fn login_user(
    db_ref: &Database,
    user: &str,
    pass: &str,
) -> Result<Option<User>, ControllerError> {
    // Attempt to find user doc
    let user_doc_result = User::find_one(db_ref, doc! { "user": user }, None).await;
    if user_doc_result.is_err() {
        return Err(ControllerError {
            io: None,
            wither: user_doc_result.err(),
            bcrypt: None,
            operation: None,
        });
    }

    // Check if user exist
    let user_doc = user_doc_result.unwrap();
    if user_doc.is_none() {
        return Ok(None);
    }

    let user = user_doc.unwrap();

    // Try to get verify password result from bcrypt
    let verify_pass_result = verify(pass, &user.pass);
    if verify_pass_result.is_err() {
        return Err(ControllerError {
            io: None,
            wither: None,
            bcrypt: verify_pass_result.err(),
            operation: None,
        });
    }

    if !verify_pass_result.unwrap() {
        return Ok(None);
    }

    // If all goes well return current user to store in memory
    Ok(Some(user))
}
/**
 * Attempt to register a new access group
 */
pub async fn register_new_access_group(
    db_ref: &Database,
    admin: &ObjectId,
    tag: &str,
) -> Result<(), ControllerError> {

    Ok(())
}
