// for testing purposes

extern crate diesel;
extern crate ipfs_proxy;

use self::ipfs_proxy::actions::*;
use self::ipfs_proxy::*;
use rand::Rng;
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
    type SaltType = [u8; 32];
    let salt_gen: SaltType = rand::thread_rng().gen::<SaltType>();
    let salt: &[u8] = &salt_gen[..];
    let salt_vec = salt.to_vec();

    let user = create_user(
        &connection,
        email.to_owned(),
        pw_hash.to_owned(),
        salt_vec.to_owned(),
    );
    println!("user: {:?}", user.unwrap().email);
}

// #[cfg(not(windows))]
// const EOF: &'static str = "CTRL+D";

// #[cfg(windows)]
// const EOF: &'static str = "CTRL+Z";
