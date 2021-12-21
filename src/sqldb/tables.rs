use crate::enums::*;

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
pub struct RpslObject {
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
