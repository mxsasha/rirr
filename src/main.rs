#[macro_use]
extern crate diesel;
extern crate dotenv;

use ipnetwork::IpNetwork;
use sqldb::handler::*;

use crate::utils::parse_asn;

mod enums;
mod sqldb;
mod utils;

fn main() {
    let connection = establish_connection();
    // let members = members_for_as_set(&connection, "AS-DASHCARE");
    // for member in &members {
    //     let Vec<i64> = match parse_asn(member) {
    //         Ok(asn) => vec![asn],
    //         Err(_e) => 2,
    //     };
    // }
    let mut already_resolved: Vec<String> = vec![];
    let members = resolve(&connection, "AS-TELIANET", &mut already_resolved);
    // let masns: Vec<i64> = members.iter().map(|m| parse_asn(m)).flatten().collect();
    let prefixes: Vec<IpNetwork> = prefixes_for_origins(&connection, &members);
    // println!("{:?}", prefixes);
    println!("{:?}", prefixes);
    println!("{:?}", members);
    // println!("{:?}", masns);
}

fn resolve(
    connection: &diesel::PgConnection,
    set_name: &str,
    already_resolved: &mut Vec<String>,
) -> Vec<i64> {
    let mut result = vec![];
    let members = members_for_as_set(connection, set_name);
    // println!("resolved {} to members {:?}", set_name, members);
    for member in &members {
        if already_resolved.contains(member) {
            return result;
        }
        already_resolved.push(member.clone());
        let mut submembers: Vec<i64> = match parse_asn(member) {
            Ok(asn) => vec![asn],
            Err(_e) => resolve(connection, member, already_resolved),
        };
        result.append(&mut submembers);
    }
    println!("resolved {} to vec {:?}", set_name, result);
    result
}
