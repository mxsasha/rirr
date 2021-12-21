table! {
    use diesel::sql_types::*;

    database_status (pk) {
        pk -> Uuid,
        source -> Varchar,
        serial_oldest_seen -> Nullable<Int4>,
        serial_newest_seen -> Nullable<Int4>,
        serial_oldest_journal -> Nullable<Int4>,
        serial_newest_journal -> Nullable<Int4>,
        serial_last_export -> Nullable<Int4>,
        force_reload -> Bool,
        last_error -> Nullable<Text>,
        last_error_timestamp -> Nullable<Timestamptz>,
        created -> Timestamptz,
        updated -> Timestamptz,
        serial_newest_mirror -> Nullable<Int4>,
        synchronised_serials -> Bool,
    }
}

table! {
    use diesel::sql_types::*;

    roa_object (pk) {
        pk -> Uuid,
        prefix -> Cidr,
        asn -> Int8,
        max_length -> Int4,
        trust_anchor -> Nullable<Varchar>,
        ip_version -> Int4,
        created -> Timestamptz,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::enums::*;

    rpsl_objects (pk) {
        pk -> Uuid,
        rpsl_pk -> Varchar,
        source -> Varchar,
        object_class -> Varchar,
        parsed_data -> Jsonb,
        object_text -> Text,
        ip_version -> Nullable<Int4>,
        ip_first -> Nullable<Inet>,
        ip_last -> Nullable<Inet>,
        ip_size -> Nullable<Numeric>,
        asn_first -> Nullable<Int8>,
        asn_last -> Nullable<Int8>,
        created -> Timestamptz,
        updated -> Timestamptz,
        rpki_status -> Rpkistatus,
        prefix_length -> Nullable<Int4>,
        scopefilter_status -> Scopefilterstatus,
        prefix -> Nullable<Cidr>,
    }
}

allow_tables_to_appear_in_same_query!(database_status, roa_object, rpsl_objects,);
