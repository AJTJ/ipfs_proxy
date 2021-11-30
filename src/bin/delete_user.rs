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

    delete_user(&connection, &email.to_owned());
}
