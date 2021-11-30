use crate::models;
use crate::schema;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use uuid::Uuid;

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn create_user(
    conn: &PgConnection,
    email_input: String,
    pw_hash_input: String,
    salt_input: String,
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
    email_input: String,
) -> Result<Option<models::Users>, DbError> {
    use crate::schema::users::dsl::*;
    let user = users
        .filter(email.eq(email_input))
        .first::<models::Users>(conn)
        .optional()?;

    Ok(user)
}

pub fn delete_user(conn: &PgConnection, email: String) {
    use schema::users;
    diesel::delete(users::table.filter(users::email.eq(email)))
        .execute(conn)
        .expect("failed to remove user");
}

pub fn return_data(conn: &PgConnection, input_id: i32) {
    use crate::schema::api_key::dsl::*;
    let api_keys = api_key.filter(user_id.eq(input_id));
}

pub fn create_new_api_key() {
    let new_uuid = Uuid::new_v4();
    // save to database
    // return nothing
}

pub fn save_api_request() {}
