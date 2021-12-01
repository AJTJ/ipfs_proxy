use crate::actions;
use crate::models::ApiKey;
use crate::models::KeyRequestDTO;
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

async fn return_data(
    pool: web::Data<DbPool>,
    email: String,
) -> Result<Vec<(ApiKey, Option<KeyRequestDTO>)>, Error> {
    let return_data = web::block(move || {
        let conn = pool.get().expect("couldn't get db connection from pool");
        actions::get_all_api_key_data(&conn, &email)
    })
    .await
    .map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    Ok(return_data)
}

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
    // need error handling for receiving incompatible data from front-end
    let body_json: SignInSignUp = serde_json::from_str(&req_body).expect("error in login body");
    let email = body_json.email;
    let password = body_json.password;

    let closure_email = email.clone();
    let closure_pool = pool.clone();
    let user = web::block(move || {
        let conn = closure_pool
            .get()
            .expect("couldn't get db connection from pool");
        actions::find_user(&conn, &closure_email)
    })
    .await
    .map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    });

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
                            let closure_email = email.clone();
                            let closure_pool = pool.clone();
                            let return_data =
                                return_data(closure_pool, closure_email).await.unwrap();

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
pub async fn delete_key(
    id: Identity,
    pool: web::Data<DbPool>,
    req_body: String,
) -> Result<impl Responder, Error> {
    // check login with actix_identity
    match id.identity() {
        Some(_) => {
            let body_json: ApiKeyRequest =
                serde_json::from_str(&req_body).expect("error in login body");
            let api_key = body_json.api_key;

            let key = web::block(move || {
                let conn = pool.get().expect("couldn't get db connection from pool");
                actions::disable_api_key(&conn, api_key)
            })
            .await
            .map_err(|e| {
                eprintln!("{}", e);
                HttpResponse::InternalServerError().finish()
            })?;

            // return all api_keys and user data
            Ok(HttpResponse::Ok().body(json!(key)))
        }
        None => Ok(HttpResponse::Forbidden().finish()),
    }
}

#[get("/getapikey")]
pub async fn get_api_key(id: Identity, pool: web::Data<DbPool>) -> Result<impl Responder, Error> {
    // check login with actix_identity
    match id.identity() {
        Some(usr_email) => {
            let closure_email = usr_email.clone();
            let closure_pool = pool.clone();
            web::block(move || {
                let conn = closure_pool
                    .get()
                    .expect("couldn't get db connection from pool");
                actions::create_new_api_key(&conn, &closure_email)
            })
            .await
            .map_err(|e| {
                eprintln!("{}", e);
                HttpResponse::InternalServerError().finish()
            })?;

            let closure_email = usr_email.clone();
            let closure_pool = pool.clone();
            let return_data = return_data(closure_pool, closure_email).await.unwrap();
            Ok(HttpResponse::Ok().body(json!(return_data)))
        }
        None => Ok(HttpResponse::Forbidden().finish()),
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
    // check login with actix_identity
    match id.identity() {
        Some(_) => {
            // need error handling for receiving incompatible data from front-end
            let body_json: ApiKeyRequest =
                serde_json::from_str(&req_body).expect("error in login body");
            let api_key = body_json.api_key;

            // get key from db NEEDS better error handling
            // let retrieved_key = get_single_api_key(&conn, api_key).unwrap().expect("no key");

            let closure_pool = pool.clone();
            let closure_key = api_key.clone();
            let retrieved_key = web::block(move || {
                let conn = closure_pool
                    .get()
                    .expect("couldn't get db connection from pool");
                actions::get_single_api_key(&conn, closure_key)
            })
            .await
            .map_err(|e| {
                eprintln!("{}", e);
                HttpResponse::InternalServerError().finish()
            })?
            .unwrap();

            if (retrieved_key.is_enabled) == true {
                // save key_request

                let closure_pool = pool.clone();
                let closure_key = api_key.clone();
                web::block(move || {
                    let conn = closure_pool
                        .get()
                        .expect("couldn't get db connection from pool");
                    actions::save_api_request(&conn, closure_key)
                })
                .await
                .map_err(|e| {
                    eprintln!("{}", e);
                    HttpResponse::InternalServerError().finish()
                })?;

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
