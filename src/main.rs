use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{middleware::Logger, App, HttpServer};
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use rand::Rng;
use std::env;

// ENDPOINTS
use ipfs_proxy::endpoints::{delete_key, echo, get_api_key, get_photo, login, logout, register};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Generate a random 32 byte key. Note that it is important to use a unique
    // private key for every project. Anyone with access to the key can generate
    // authentication cookies for any user!
    let private_key = rand::thread_rng().gen::<[u8; 32]>();

    dotenv().ok();
    let connspec = env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(connspec);

    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&private_key)
                    .name("auth-example")
                    .secure(false),
            ))
            .data(pool.clone())
            // enable logger - always register actix-web Logger middleware last
            .wrap(Logger::default())
            .service(register)
            .service(login)
            .service(logout)
            .service(delete_key)
            .service(get_api_key)
            .service(get_photo)
            .service(echo)
    })
    .bind("127.0.0.1:8090")?
    .run()
    .await
}
