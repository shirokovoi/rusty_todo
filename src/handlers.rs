use crate::{
    errors::Error,
    models::{api::*, internal::*},
    repository::Repository,
};

use actix_web::{web, HttpResponse};
use actix_web_httpauth::extractors::basic::BasicAuth;
use bcrypt;

async fn check_credentials(
    auth: &BasicAuth,
    repository: &web::Data<Repository>,
) -> Result<UserIdentity, Error> {
    let password = auth.password().ok_or(Error::Unauthorized)?;

    repository
        .check_credentials(auth.user_id(), |hashed_password| -> Result<bool, Error> {
            bcrypt::verify(password, hashed_password).map_err(|err| err.into())
        })
        .await
}

pub async fn user_register(
    repository: web::Data<Repository>,
    info: web::Json<UserInfo>,
) -> Result<HttpResponse, Error> {
    let crypted = bcrypt::hash_with_result(&info.password, bcrypt::DEFAULT_COST)?; // FIXME long blocking operation!
    repository
        .create_user(&info.username, &crypted.to_string())
        .await?;

    Ok(HttpResponse::Ok().into())
}

pub async fn get_my_list(
    repository: web::Data<Repository>,
    auth: BasicAuth,
) -> Result<web::Json<ListId>, Error> {
    let identity = check_credentials(&auth, &repository).await?;
    let list_id = repository.get_user_list(&identity).await?;

    Ok(web::Json(ListId { list_id }))
}

pub async fn get_all_list_ids(
    repository: web::Data<Repository>,
) -> Result<web::Json<ListIds>, Error> {
    let list_ids = repository.get_all_list_ids().await?;
    Ok(web::Json(ListIds { list_ids }))
}

pub async fn create_list(
    repository: web::Data<Repository>,
    auth: BasicAuth,
) -> Result<web::Json<ListId>, Error> {
    let identity = check_credentials(&auth, &repository).await?;
    let list_id = repository.create_list(&identity).await?;

    Ok(web::Json(ListId { list_id }))
}

pub async fn modify_entry_order(
    repository: web::Data<Repository>,
    list_id: web::Path<u32>,
    modify: web::Json<ListOrderModify>,
    auth: BasicAuth,
) -> Result<HttpResponse, Error> {
    let identity = check_credentials(&auth, &repository).await?;

    let priorities = modify
        .priorities
        .iter()
        .map(|priority| EntryPriority {
            entry_id: priority.entry_id,
            priority: priority.priority,
        })
        .collect();

    repository
        .modify_entry_order(&identity, *list_id, modify.version, priorities)
        .await?;

    Ok(HttpResponse::Ok().into())
}

pub async fn get_list(
    repository: web::Data<Repository>,
    list_id: web::Path<u32>,
    paging: web::Query<PagingParameters>,
) -> Result<web::Json<ListResponse>, Error> {
    let list = repository
        .get_list(*list_id, paging.count, paging.offset)
        .await?;

    let entries = list
        .entries
        .iter()
        .map(|entry| TodoEntry {
            id: entry.id,
            priority: entry.priority,
            description: entry.value.clone(), // TODO use move
        })
        .collect();

    Ok(web::Json(ListResponse {
        version: list.version,
        total_entries: list.entiries_count,
        entries,
    }))
}

pub async fn delete_list(
    repository: web::Data<Repository>,
    list_id: web::Path<u32>,
    auth: BasicAuth,
) -> Result<HttpResponse, Error> {
    let identity = check_credentials(&auth, &repository).await?;
    repository.delete_list(&identity, *list_id).await?;

    Ok(HttpResponse::Ok().into())
}

pub async fn add_entry(
    repository: web::Data<Repository>,
    list_id: web::Path<u32>,
    entry: web::Json<EntryCreate>,
    auth: BasicAuth,
) -> Result<HttpResponse, Error> {
    let identity = check_credentials(&auth, &repository).await?;
    repository
        .add_entry(&identity, *list_id, entry.version, &entry.value)
        .await?;

    Ok(HttpResponse::Ok().into())
}

pub async fn delete_entry(
    auth: BasicAuth,
    repository: web::Data<Repository>,
    location: web::Path<EntryLocation>,
) -> Result<HttpResponse, Error> {
    let identity = check_credentials(&auth, &repository).await?;
    repository
        .delete_entry(
            &identity,
            location.version,
            location.list_id,
            location.entry_id,
        )
        .await?;

    Ok(HttpResponse::Ok().into())
}
