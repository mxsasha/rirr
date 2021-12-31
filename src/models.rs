use async_graphql::{ComplexObject, SimpleObject};
use chrono::{DateTime, Utc};

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

#[derive(Queryable, Debug, SimpleObject)]
#[graphql(complex)]
pub struct RpslObject {
    pub pk: uuid::Uuid,
    pub rpsl_pk: String,
    pub source: String,
    pub object_class: String,
    pub parsed_data: serde_json::Value,
    pub object_text: String,
    pub ip_version: Option<i32>,
    #[graphql(skip)]
    pub ip_first: Option<ipnetwork::IpNetwork>,
    #[graphql(skip)]
    pub ip_last: Option<ipnetwork::IpNetwork>,
    #[graphql(skip)]
    pub ip_size: Option<bigdecimal::BigDecimal>,
    pub asn_first: Option<i64>,
    pub asn_last: Option<i64>,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub rpki_status: RPKIStatus,
    pub prefix_length: Option<i32>,
    pub scopefilter_status: ScopeFilterStatus,
    #[graphql(skip)]
    pub prefix: Option<ipnetwork::IpNetwork>,
}

#[ComplexObject]
impl RpslObject {
    #[graphql(name = "ipFirst")]
    pub async fn ip_first_str(&self) -> Option<String> {
        self.ip_first.map(|n| n.to_string())
    }
    #[graphql(name = "ipLast")]
    pub async fn ip_last_str(&self) -> Option<String> {
        self.ip_last.map(|n| n.to_string())
    }
    #[graphql(name = "prefix")]
    pub async fn prefix_str(&self) -> Option<String> {
        self.prefix.map(|n| n.to_string())
    }
    #[graphql(name = "ipSize")]
    pub async fn ip_size_str(&self) -> Option<String> {
        self.ip_size.as_ref().map(|n| n.to_string())
    }
}

#[derive(Queryable, Debug)]
pub struct RpslObjectRoute {
    pub source: String,
    pub asn_first: Option<i64>,
    pub prefix: Option<ipnetwork::IpNetwork>,
}
