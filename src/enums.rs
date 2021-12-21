#[derive(diesel_derive_enum::DbEnum, Debug)]
#[DieselType = "Scopefilterstatus"]
pub enum ScopeFilterStatus {
    InScope,
    OutOfScopeAs,
    OutOfScopePrefix,
}

#[derive(diesel_derive_enum::DbEnum, Debug)]
#[DieselType = "Rpkistatus"]
pub enum RPKIStatus {
    Valid,
    Invalid,
    NotFound,
}
