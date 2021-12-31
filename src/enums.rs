#[derive(async_graphql::Enum, diesel_derive_enum::DbEnum, Debug, Copy, Clone, PartialEq, Eq)]
#[DieselType = "Scopefilterstatus"]
pub enum ScopeFilterStatus {
    InScope,
    OutOfScopeAs,
    OutOfScopePrefix,
}

#[derive(async_graphql::Enum, diesel_derive_enum::DbEnum, Debug, Copy, Clone, PartialEq, Eq)]
#[DieselType = "Rpkistatus"]
pub enum RPKIStatus {
    Valid,
    Invalid,
    NotFound,
}
