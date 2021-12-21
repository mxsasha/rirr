#[macro_use]
extern crate diesel;
extern crate dotenv;

use ipnetwork::IpNetwork;
use sqldb::handler::*;

mod enums;
mod sqldb;
mod utils;

fn main() {
    let connection = establish_connection();
    let members = members_for_as_set_recursive(&connection, "AS-TELIANET");
    println!("{:?}", members);
    let prefixes: Vec<IpNetwork> = prefixes_for_origins(&connection, &members);
    println!("{:?}", prefixes);
}
