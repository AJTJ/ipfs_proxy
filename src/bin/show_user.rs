// for testing purposes

extern crate diesel;
extern crate ipfs_proxy;

use self::diesel::prelude::*;
use self::ipfs_proxy::*;
use self::models::*;
fn main() {
    use ipfs_proxy::schema::users::dsl::*;
    let connection = establish_connection();

    let results = users
        .load::<Users>(&connection)
        .expect("Error loading users");

    for user in results {
        println!("{}, {}, {:?}", user.email, user.pw_hash, user.salt);
    }
}
