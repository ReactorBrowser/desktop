// @generated automatically by Diesel CLI.

diesel::table! {
    history (id) {
        id -> Integer,
        url -> Text,
        title -> Nullable<Text>,
        visited_at -> Timestamp,
    }
}

diesel::table! {
    tabs (id) {
        id -> Integer,
        url -> Text,
        title -> Nullable<Text>,
        loaded -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    history,
    tabs,
);
