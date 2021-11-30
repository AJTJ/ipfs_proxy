use crate::actions::{self, find_user, return_data};
use actix_identity::Identity;
use actix_web::{get, post, web, Error, HttpRequest, HttpResponse, Responder};
use argon2::{self, Config};
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use rand::Rng;
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
    req: HttpRequest,
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
    let converted_salt = str::from_utf8(salt).unwrap();

    // save email and password in database
    // use web::block to offload blocking Diesel code without blocking server thread
    let email_clone = email.clone();
    let converted_salt = converted_salt.to_owned();
    let user = web::block(move || {
        let conn = pool.get().expect("couldn't get db connection from pool");
        actions::create_user(&conn, email_clone, password_hash, converted_salt)
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
pub async fn login(
    id: Identity,
    req: HttpRequest,
    req_body: String,
    pool: web::Data<DbPool>,
) -> impl Responder {
    let conn = pool.get().expect("couldn't get db connection from pool");

    // need error handling for receiving incompatible data from front-end
    let body_json: SignInSignUp = serde_json::from_str(&req_body).expect("error in login body");
    let email = body_json.email;
    let password = body_json.password;

    // get user
    let user = find_user(&conn, email.to_owned());

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
                            let return_data = return_data(&conn, user_record.id);

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
#[post("/deletekey")]
pub async fn delete_key(id: Identity, api_key: String) -> impl Responder {
    // check login with actix_identity
    match id.identity() {
        Some(_) => {
            // remove api key from database
            // return all api_keys and user data
            HttpResponse::Ok().body("Should delete a key here")
        }
        None => HttpResponse::Forbidden().finish(),
    }
}

#[get("/getapikey")]
pub async fn get_api_key(id: Identity) -> impl Responder {
    // check login with actix_identity
    match id.identity() {
        Some(_) => {
            // generate new uuid
            // save to database
            // return all api_keys and user data
            HttpResponse::Ok().body("Should get a new api key here")
        }
        None => HttpResponse::Forbidden().finish(),
    }
}

// NODE REQUESTS
#[post("/getphoto")]
pub async fn get_photo(id: Identity) -> impl Responder {
    // check login with actix_identity
    match id.identity() {
        Some(_) => {
            // save key_request
            // process request
            // return response
            // return all api_keys and user data
            HttpResponse::Ok().body("Should request a photo from the node with an api key here")
        }
        None => HttpResponse::Forbidden().finish(),
    }
}

// TEST ENDPOINT
#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
