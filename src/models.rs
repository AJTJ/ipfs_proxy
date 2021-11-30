use chrono::prelude::*;
use diesel::{sql_types::Timestamp, Insertable, Queryable};
use serde::{Deserialize, Serialize};

use super::schema::{api_key, key_requests, users};
use uuid;

#[derive(Queryable)]
pub struct Users {
    pub id: i32,
    pub email: String,
    pub pw_hash: String,
    pub salt: Vec<u8>,
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub email: String,
    pub pw_hash: String,
    pub salt: Vec<u8>,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct ApiKey {
    pub id: i32,
    pub user_id: i32,
    pub key_value: uuid::Uuid,
    pub is_enabled: bool,
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "api_key"]
pub struct NewApiKey {
    pub user_id: i32,
    pub key_value: uuid::Uuid,
    pub is_enabled: bool,
}

#[derive(Queryable)]
pub struct KeyRequests {
    pub id: i32,
    pub api_key_id: i32,
    pub date_time: Timestamp,
}

#[derive(Serialize, Deserialize, Queryable)]
pub struct KeyRequestDTO {
    pub id: i32,
    pub api_key_id: i32,
    pub date_time: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "key_requests"]
pub struct NewKeyRequest {
    pub api_key_id: i32,
    pub date_time: NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct KeysWithRequests {
    pub user_id: i32,
    pub key_value: uuid::Uuid,
    pub is_enabled: bool,
    pub requests: Vec<NewKeyRequest>,
}
