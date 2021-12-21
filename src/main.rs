mod tables;
mod enums;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use enums::{ScopeFilterStatus, RPKIStatus};
use std::env;

#[derive(Queryable, Debug)]
pub struct Roa {
    pub pk: uuid::Uuid,
    pub prefix: ipnetwork::IpNetwork,
    pub asn: i64,
    pub max_length: i32,
    pub trust_anchor: Option<String>,
    pub ip_version: i32,
    pub created: diesel::data_types::PgTimestamp,
}

#[derive(Queryable, Debug)]
pub struct RpslObjects {
    pub pk: uuid::Uuid,
    pub rpsl_pk: String,
    pub source: String,
    pub object_class: String,
    pub parsed_data: serde_json::Value,
    pub object_text: String,
    pub ip_version: Option<i32>,
    pub ip_first: Option<ipnetwork::IpNetwork>,
    pub ip_last: Option<ipnetwork::IpNetwork>,
    pub ip_size: Option<bigdecimal::BigDecimal>,
    pub asn_first: Option<i64>,
    pub asn_last: Option<i64>,
    pub created: diesel::data_types::PgTimestamp,
    pub updated: diesel::data_types::PgTimestamp,
    pub rpki_status: RPKIStatus,
    pub prefix_length: Option<i32>,
    pub scopefilter_status: ScopeFilterStatus,
    pub prefix: Option<ipnetwork::IpNetwork>,
}

#[derive(Queryable, Debug)]
pub struct RpslObjectsNarrow {
    pub prefix: Option<ipnetwork::IpNetwork>,
    pub asn_first: Option<i64>,
    pub source: String,
}

fn main() {
    use crate::tables::*;

    let connection = establish_connection();
    let results = rpsl_objects::table
        // .select((rpsl_objects::prefix, rpsl_objects::asn_first, rpsl_objects::source))
        // .filter(rpsl_objects::source.eq("NTTCOM"))
        .limit(50)
        .load::<RpslObjects>(&connection)
        .expect("Error loading posts");
    println!("{:?}", results);
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
