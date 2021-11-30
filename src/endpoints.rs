use crate::actions::{
    self, create_new_api_key, disable_api_key, find_user, get_all_api_key_data, get_single_api_key,
    save_api_request,
};
use actix_identity::Identity;
use actix_web::{get, post, web, Error, HttpResponse, Responder};
use argon2::{self, Config};
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use rand::Rng;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::str;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct SignInSignUp {
    email: String,
    password: String,
}

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
type SaltType = [u8; 32];

// AUTH ENDPOINTS
#[post("/register")]
pub async fn register(
    id: Identity,
    req_body: String,
    pool: web::Data<DbPool>,
) -> Result<impl Responder, Error> {
    // need error handling for receiving incompatible data from front-end
    let body_json: SignInSignUp = serde_json::from_str(&req_body).expect("error in login body");
    let email = body_json.email;
    let password = body_json.password;

    //TODO: would need to check if a user with that email already exists

    // argon things
    let config = Config::default();
    let salt_gen: SaltType = rand::thread_rng().gen::<SaltType>();
    let salt: &[u8] = &salt_gen[..];
    let password_hash = argon2::hash_encoded(password.as_bytes(), &salt, &config).unwrap();

    // save email and password in database
    // use web::block to offload blocking Diesel code without blocking server thread
    let salt_vec = salt.to_vec();
    let email_clone = email.clone();
    let user = web::block(move || {
        let conn = pool.get().expect("couldn't get db connection from pool");
        actions::create_user(&conn, email_clone, password_hash, salt_vec)
    })
    .await
    .map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    // login
    id.remember(email.to_owned());

    Ok(HttpResponse::Ok().json(user.email))
}

#[post("/login")]
pub async fn login(id: Identity, req_body: String, pool: web::Data<DbPool>) -> impl Responder {
    let conn = pool.get().expect("couldn't get db connection from pool");

    // need error handling for receiving incompatible data from front-end
    let body_json: SignInSignUp = serde_json::from_str(&req_body).expect("error in login body");
    let email = body_json.email;
    let password = body_json.password;

    // get user
    let user = find_user(&conn, &email);

    match user {
        Ok(usr) => {
            match usr {
                Some(user_record) => {
                    // verify password matches
                    let pw_hash = user_record.pw_hash;
                    let password_match =
                        argon2::verify_encoded(&pw_hash, password.as_bytes()).unwrap();

                    match password_match {
                        // CORRECT PW AND LOGIN
                        true => {
                            // login with actix_identity
                            id.remember(email.to_owned());

                            // return all api_keys and user data
                            let return_data = get_all_api_key_data(&conn, &email);

                            HttpResponse::Ok().body(json!(return_data))
                        }
                        // WRONG PASSWORD
                        false => HttpResponse::Forbidden().finish(),
                    }
                }
                None => HttpResponse::BadRequest().finish(),
            }
        }
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}

#[post("/logout")]
pub async fn logout(id: Identity) -> impl Responder {
    // logout with actix_identity
    id.forget();
    HttpResponse::Ok().body("Should logout here")
}

// API KEY ENDPOINTS
#[post("/disablekey")]
pub async fn delete_key(id: Identity, pool: web::Data<DbPool>, req_body: String) -> impl Responder {
    // check login with actix_identity
    match id.identity() {
        Some(_) => {
            let conn = pool.get().expect("couldn't get db connection from pool");
            let body_json: ApiKeyRequest =
                serde_json::from_str(&req_body).expect("error in login body");
            let api_key = body_json.api_key;
            let key = disable_api_key(&conn, api_key).unwrap();
            // return all api_keys and user data
            HttpResponse::Ok().body(json!(key))
        }
        None => HttpResponse::Forbidden().finish(),
    }
}

#[get("/getapikey")]
pub async fn get_api_key(id: Identity, pool: web::Data<DbPool>) -> impl Responder {
    let conn = pool.get().expect("couldn't get db connection from pool");

    // check login with actix_identity
    match id.identity() {
        Some(usr_email) => {
            create_new_api_key(&conn, &usr_email).unwrap();
            let all_data = get_all_api_key_data(&conn, &usr_email);
            HttpResponse::Ok().body(json!(all_data))
        }
        None => HttpResponse::Forbidden().finish(),
    }
}

#[derive(Serialize, Deserialize)]
pub struct ApiKeyRequest {
    api_key: Uuid,
}

// NODE REQUESTS
#[post("/interactnode")]
pub async fn get_photo(
    id: Identity,
    pool: web::Data<DbPool>,
    req_body: String,
) -> Result<impl Responder, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    // check login with actix_identity
    match id.identity() {
        Some(_) => {
            // need error handling for receiving incompatible data from front-end
            let body_json: ApiKeyRequest =
                serde_json::from_str(&req_body).expect("error in login body");
            let api_key = body_json.api_key;

            // get key from db NEEDS better error handling
            let retrieved_key = get_single_api_key(&conn, api_key).unwrap().expect("no key");
            if retrieved_key.is_enabled == true {
                // save key_request
                save_api_request(&conn, api_key).unwrap();
                // process node request
                let client = reqwest::Client::new();
                let res = client
                    .post("http://127.0.0.1:5001/api/v0/bitswap/reprovide")
                    .body("the exact body that is sent")
                    .send()
                    .await
                    .expect("unsuccessful node interaction")
                    .text()
                    .await
                    .expect("no text");

                // return response
                // return all api_keys and user data
                Ok(HttpResponse::Ok().body(json!(res)))
            } else {
                Ok(HttpResponse::BadRequest().body(json!("key is disabled")))
            }
        }
        None => Ok(HttpResponse::Forbidden().finish()),
    }
}

// TEST ENDPOINT
#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
