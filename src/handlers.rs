use crate::{errors::Error, models::*, repository};

use crate::repository::Repository;
use actix_web::{web, HttpResponse};
use actix_web_httpauth::extractors::basic::BasicAuth;
use bcrypt;

pub async fn user_register(
    repository: web::Data<Repository>,
    info: web::Json<UserInfo>,
) -> Result<HttpResponse, Error> {
    let crypted = bcrypt::hash_with_result(&info.password, bcrypt::DEFAULT_COST)?; // FIXME long
                                                                                   // blocking
                                                                                   // operation!

    repository
        .create_user(&info.username, &crypted.to_string())
        .await?;

    Ok(HttpResponse::Ok().into())
}

pub async fn get_my_list(
    repository: web::Data<Repository>,
    auth: BasicAuth,
) -> Result<web::Json<ListId>, Error> {
    unimplemented!();
}

pub async fn get_all_list_ids(
    repository: web::Data<Repository>,
) -> Result<web::Json<ListIds>, Error> {
    unimplemented!();
}

pub async fn create_list(
    repository: web::Data<Repository>,
    auth: BasicAuth,
) -> Result<web::Json<ListId>, Error> {
    unimplemented!()
}

pub async fn modify_entry_order(
    repository: web::Data<Repository>,
    list_id: web::Path<u32>,
    modify: web::Json<ListOrderModify>,
) -> Result<HttpResponse, Error> {
    unimplemented!()
}

pub async fn get_list(
    repository: web::Data<Repository>,
    list_id: web::Path<u32>,
    paging: web::Query<PagingParameters>,
) -> Result<web::Json<ListResponse>, Error> {
    unimplemented!()
}

pub async fn delete_list(
    repository: web::Data<Repository>,
    list_id: web::Path<u32>,
) -> Result<HttpResponse, Error> {
    unimplemented!();
}

pub async fn add_entry(
    repository: web::Data<Repository>,
    list_id: web::Path<u32>,
    entry: web::Json<EntryCreate>,
) -> Result<HttpResponse, Error> {
    unimplemented!();
}

pub async fn delete_entry(
    repository: web::Data<Repository>,
    ids: web::Path<(u32, u32)>,
) -> Result<HttpResponse, Error> {
    unimplemented!()
}
