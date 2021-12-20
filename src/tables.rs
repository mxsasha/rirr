table! {
    alembic_version (version_num) {
        version_num -> Varchar,
    }
}

table! {
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
    rpsl_database_journal (pk) {
        pk -> Uuid,
        rpsl_pk -> Varchar,
        source -> Varchar,
        serial_nrtm -> Int4,
        operation -> Varchar,
        object_class -> Varchar,
        object_text -> Text,
        timestamp -> Timestamptz,
        origin -> Varchar,
    }
}

table! {
    rpsl_objects (pk) {
        pk -> Uuid,
        rpsl_pk -> Varchar,
        source -> Varchar,
        object_class -> Varchar,
        parsed_data -> Jsonb,
        // object_text -> Text,
        ip_version -> Nullable<Int4>,
        ip_first -> Nullable<Inet>,
        ip_last -> Nullable<Inet>,
        ip_size -> Nullable<Numeric>,
        asn_first -> Nullable<Int8>,
        asn_last -> Nullable<Int8>,
        created -> Timestamptz,
        updated -> Timestamptz,
        rpki_status -> Varchar,
        prefix_length -> Nullable<Int4>,
        scopefilter_status -> Varchar,
        prefix -> Nullable<Cidr>,
    }
}

table! {
    rpsl_objects_suspended (pk) {
        pk -> Uuid,
        rpsl_pk -> Varchar,
        source -> Varchar,
        object_class -> Varchar,
        object_text -> Text,
        mntners -> Array<Text>,
        timestamp -> Timestamptz,
        original_created -> Timestamptz,
        original_updated -> Timestamptz,
    }
}

allow_tables_to_appear_in_same_query!(
    alembic_version,
    database_status,
    roa_object,
    rpsl_database_journal,
    rpsl_objects,
    rpsl_objects_suspended,
);
