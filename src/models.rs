use diesel::{
    sql_types::{Timestamp, Uuid},
    Insertable, Queryable,
};
use serde::{Deserialize, Serialize};

use super::schema::users;

#[derive(Queryable)]
pub struct Users {
    pub id: i32,
    pub email: String,
    pub pw_hash: String,
    pub salt: String,
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub email: String,
    pub pw_hash: String,
    pub salt: String,
}

#[derive(Queryable)]
pub struct ApiKey {
    pub id: i32,
    pub user_id: i32,
    pub key_value: Uuid,
    pub is_enabled: bool,
}

#[derive(Queryable)]
pub struct KeyRequests {
    pub id: i32,
    pub api_key_id: i32,
    pub date_time: Timestamp,
    pub successful: bool,
}
