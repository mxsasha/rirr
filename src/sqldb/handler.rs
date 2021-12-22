use crate::utils::parse_asn;

use super::schema::*;
use super::tables::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use ipnetwork::IpNetwork;
use std::collections::HashMap;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_prefix_store(conn: &PgConnection) -> HashMap<i64, Vec<IpNetwork>> {
    let rpsl_objs = rpsl_objects::table
        .select((rpsl_objects::source, rpsl_objects::asn_first, rpsl_objects::prefix))
        .filter(rpsl_objects::object_class.eq_any(["route", "route6"]))
        .load::<RpslObjectRoute>(conn)
        .expect("Error loading prefixes");

    let mut result: HashMap<i64, Vec<IpNetwork>> = HashMap::new();
    for rpsl_obj in rpsl_objs.into_iter() {
        result.entry(rpsl_obj.asn_first.unwrap()).or_default().push(rpsl_obj.prefix.unwrap());
    }
    result
}

pub fn prefixes_for_origins(conn: &PgConnection, origins: &[i64], prefix_store: &HashMap<i64, Vec<IpNetwork>>) -> Vec<IpNetwork> {
    origins.iter().flat_map(|asn| prefix_store.get(asn)).flatten().cloned().collect()
    // let rpsl_objs = rpsl_objects::table
    //     // .select((rpsl_objects::prefix, rpsl_objects::asn_first, rpsl_objects::source))
    //     .filter(rpsl_objects::asn_first.eq_any(origins))
    //     .filter(rpsl_objects::object_class.eq_any(["route", "route6"]))
    //     .load::<RpslObject>(conn)
    //     .expect("Error loading prefixes");
    // rpsl_objs
    //     .into_iter()
    //     .flat_map(|rpsl_obj| rpsl_obj.prefix)
    //     .collect()
}

pub fn members_for_as_set(conn: &PgConnection, set_name: &str) -> Vec<String> {
    let rpsl_objs = rpsl_objects::table
        .filter(rpsl_objects::rpsl_pk.eq(set_name))
        .filter(rpsl_objects::object_class.eq("as-set"))
        .load::<RpslObject>(conn)
        .expect("Error loading prefixes");

    rpsl_objs
        .into_iter()
        .flat_map(|rpsl_obj| {
            rpsl_obj.parsed_data["members"]
                .as_array()
                .unwrap_or(&vec![])
                .iter()
                .map(|m| String::from(m.as_str().unwrap_or_default()))
                .collect::<Vec<String>>()
        })
        .collect()
}

pub fn members_for_as_set_recursive(connection: &PgConnection, set_name: &str) -> Vec<i64> {
    let mut already_resolved: Vec<String> = vec![];
    members_for_as_set_recursive_internal(connection, set_name, &mut already_resolved)
}

fn members_for_as_set_recursive_internal(
    connection: &PgConnection,
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
            Err(_e) => members_for_as_set_recursive_internal(connection, member, already_resolved),
        };
        result.append(&mut submembers);
    }
    println!("resolved {} to vec {:?}", set_name, result);
    result
}
