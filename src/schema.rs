// @generated automatically by Diesel CLI.

diesel::table! {
    history (id) {
        id -> Integer,
        url -> Text,
        title -> Nullable<Text>,
        visited_at -> Timestamp,
    }
}
