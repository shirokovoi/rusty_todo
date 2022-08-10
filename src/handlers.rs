use crate::errors::Error;
use actix_web::{web, Responder};
use actix_web_httpauth::extractors::basic::BasicAuth;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct UserInfo {
    username: String,
    password: String,
}

async fn user_register(payload: web::Json<UserInfo>) -> Result<(), Error> {
    unimplemented!()
}

#[derive(Serialize)]
struct ListId {
    list_id: u32,
}

async fn get_my_list(auth: BasicAuth) -> Result<ListId, Error> {
    unimplemented!();
}

#[derive(Serialize)]
struct ListIds {
    list_ids: Vec<u32>,
}

async fn get_all_list_ids() -> Result<ListIds, Error> {
    unimplemented!();
}

async fn create_list(auth: BasicAuth) -> Result<ListId, Error> {
    unimplemented!()
}

#[derive(Serialize)]
struct Priorities {
    entry_id: u32,
    priority: u32,
}

#[derive(Serialize)]
struct ListOrderModify {
    version: u32,
    priorities: Vec<Priorities>,
}

async fn modify_entry_order(
    list_id: web::Path<u32>,
    modify: web::Json<ListOrderModify>,
) -> Result<(), Error> {
    unimplemented!()
}

struct PagingParameters {
    count: u32,
    offset: u32,
}

#[derive(Serialize)]
struct TodoEntry {
    id: u32,
    priority: u32,
    description: u32,
}

#[derive(Serialize)]
struct ListResponse {
    version: u32,
    total_entries: u32,
    entries: Vec<TodoEntry>,
}

async fn get_list(
    list_id: web::Path<u32>,
    paging: web::Query<PagingParameters>,
) -> Result<ListResponse, Error> {
    unimplemented!()
}

async fn delete_list(list_id: web::Path<u32>) -> Result<(), Error> {
    unimplemented!();
}

#[derive(Serialize)]
struct EntryCreate {
    value: String,
}

async fn add_entry(list_id: web::Path<u32>, entry: web::Json<EntryCreate>) -> Result<(), Error> {
    unimplemented!();
}

async fn delete_entry(ids: web::Path<(u32, u32)>) -> Result<(), Error> {
    unimplemented!()
}
