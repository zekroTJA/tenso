// @generated automatically by Diesel CLI.

diesel::table! {
    auth (username) {
        username -> Varchar,
        password_hash -> Varchar,
    }
}

diesel::table! {
    links (id) {
        id -> Varchar,
        ident -> Text,
        creator_id -> Varchar,
        created_date -> Timestamp,
        destination -> Text,
        enabled -> Bool,
        permanent_redirect -> Bool,
    }
}

diesel::table! {
    stats (id) {
        id -> Varchar,
        link_id -> Varchar,
        created_date -> Timestamp,
        user_agent -> Nullable<Text>,
    }
}

diesel::joinable!(links -> auth (creator_id));
diesel::joinable!(stats -> links (link_id));

diesel::allow_tables_to_appear_in_same_query!(
    auth,
    links,
    stats,
);
