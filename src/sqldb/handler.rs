use super::schema::*;
use super::tables::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use ipnetwork::IpNetwork;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn prefixes_for_origin(conn: &PgConnection, origin: i64) -> Vec<IpNetwork> {
    let rpsl_objs = rpsl_objects::table
        // .select((rpsl_objects::prefix, rpsl_objects::asn_first, rpsl_objects::source))
        .filter(rpsl_objects::asn_first.eq(origin))
        .filter(rpsl_objects::object_class.eq_any(["route", "route6"]))
        .load::<RpslObject>(conn)
        .expect("Error loading prefixes");
    rpsl_objs
        .into_iter()
        .flat_map(|rpsl_obj| rpsl_obj.prefix)
        .collect()
}

pub fn prefixes_for_origins(conn: &PgConnection, origins: &[i64]) -> Vec<IpNetwork> {
    let rpsl_objs = rpsl_objects::table
        // .select((rpsl_objects::prefix, rpsl_objects::asn_first, rpsl_objects::source))
        .filter(rpsl_objects::asn_first.eq_any(origins))
        .filter(rpsl_objects::object_class.eq_any(["route", "route6"]))
        .load::<RpslObject>(conn)
        .expect("Error loading prefixes");
    rpsl_objs
        .into_iter()
        .flat_map(|rpsl_obj| rpsl_obj.prefix)
        .collect()
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
