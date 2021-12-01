use crate::models::{self, KeyRequestDTO, NewKeyRequest};
use crate::models::{ApiKey, NewApiKey};
use crate::schema;
use chrono::prelude::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use uuid::Uuid;

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn create_user(
    conn: &PgConnection,
    email_input: String,
    pw_hash_input: String,
    salt_input: Vec<u8>,
) -> Result<models::NewUser, DbError> {
    use schema::users::dsl::*;

    let new_user = models::NewUser {
        email: email_input,
        pw_hash: pw_hash_input,
        salt: salt_input,
    };

    diesel::insert_into(users).values(&new_user).execute(conn)?;

    Ok(new_user)
}

pub fn find_user(
    conn: &PgConnection,
    input_email: &String,
) -> Result<Option<models::Users>, DbError> {
    use crate::schema::users::dsl::*;
    let user = users
        .filter(email.eq(input_email))
        .first::<models::Users>(conn)
        .optional()?;

    Ok(user)
}

pub fn delete_user(conn: &PgConnection, input_email: &String) {
    use schema::users::dsl::*;
    diesel::delete(users.filter(email.eq(input_email)))
        .execute(conn)
        .expect("failed to remove user");
}

pub fn get_all_api_key_data(
    conn: &PgConnection,
    input_email: &String,
) -> Result<Vec<(ApiKey, Option<KeyRequestDTO>)>, DbError> {
    use schema::api_key;
    use schema::key_requests;

    let user = find_user(conn, input_email)
        .unwrap()
        .expect("user not found");

    let data: Vec<(ApiKey, Option<KeyRequestDTO>)> = api_key::dsl::api_key
        .filter(api_key::dsl::user_id.eq(user.id))
        .left_join(key_requests::dsl::key_requests)
        .load(conn)
        .expect("failed to load data");

    Ok(data)
}

pub fn create_new_api_key(
    conn: &PgConnection,
    input_email: &String,
) -> Result<models::NewApiKey, DbError> {
    // handle no user found
    let user = find_user(conn, input_email).unwrap().expect("no user");
    use schema::api_key::dsl::*;
    let new_uuid = Uuid::new_v4();
    let new_api_key = NewApiKey {
        user_id: user.id,
        key_value: new_uuid,
        is_enabled: true,
    };
    // save to database
    diesel::insert_into(api_key)
        .values(&new_api_key)
        .execute(conn)?;

    Ok(new_api_key)
}

pub fn disable_api_key(conn: &PgConnection, input_key: Uuid) -> Result<Uuid, DbError> {
    use schema::api_key::dsl::*;
    diesel::update(api_key.filter(key_value.eq(input_key)))
        .set(is_enabled.eq(false))
        .execute(conn)?;

    Ok(input_key)
}

pub fn enable_api_key(conn: &PgConnection, input_key: Uuid) -> Result<Uuid, DbError> {
    use schema::api_key::dsl::*;
    diesel::update(api_key.filter(key_value.eq(input_key)))
        .set(is_enabled.eq(true))
        .execute(conn)?;

    Ok(input_key)
}

pub fn delete_api_key(conn: &PgConnection, input_key: Uuid) -> Result<Uuid, DbError> {
    use schema::api_key::dsl::*;

    diesel::delete(api_key.filter(key_value.eq(input_key))).execute(conn)?;
    Ok(input_key)
}

pub fn get_single_api_key(conn: &PgConnection, input_key: Uuid) -> Result<Option<ApiKey>, DbError> {
    use schema::api_key::dsl::*;

    let key = api_key
        .filter(key_value.eq(input_key))
        .first::<models::ApiKey>(conn)
        .optional()?;
    Ok(key)
}

pub fn save_api_request(
    conn: &PgConnection,
    input_key: Uuid,
) -> Result<models::NewKeyRequest, DbError> {
    use schema::key_requests::dsl::*;

    let key = get_single_api_key(conn, input_key)
        .unwrap()
        .expect("key not found");

    let naive_date_time = Utc::now().naive_utc();
    let new_request = NewKeyRequest {
        api_key_id: key.id,
        date_time: naive_date_time,
    };

    diesel::insert_into(key_requests)
        .values(&new_request)
        .execute(conn)?;

    Ok(new_request)
}
