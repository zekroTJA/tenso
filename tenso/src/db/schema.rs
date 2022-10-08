// @generated automatically by Diesel CLI.

diesel::table! {
    auth (username) {
        username -> Varchar,
        password_hash -> Varchar,
    }
}
