// for testing purposes

extern crate diesel;
extern crate ipfs_proxy;

use self::ipfs_proxy::actions::*;
use self::ipfs_proxy::*;
use std::io::stdin;

fn main() {
    let connection = establish_connection();

    println!("What would you like your email to be?");
    let mut email = String::new();
    stdin().read_line(&mut email).unwrap();
    let email = &email[..(email.len() - 1)]; // Drop the newline character
    let mut pw_hash = String::new();
    stdin().read_line(&mut pw_hash).unwrap();
    let pw_hash = &pw_hash[..(pw_hash.len() - 1)]; // Drop the newline character
    let mut salt = String::new();
    stdin().read_line(&mut salt).unwrap();
    let salt = &salt[..(salt.len() - 1)]; // Drop the newline character

    let user = create_user(
        &connection,
        email.to_owned(),
        pw_hash.to_owned(),
        salt.to_owned(),
    );
    println!("\nSaved draft {}", email);
}

// #[cfg(not(windows))]
// const EOF: &'static str = "CTRL+D";

// #[cfg(windows)]
// const EOF: &'static str = "CTRL+Z";
