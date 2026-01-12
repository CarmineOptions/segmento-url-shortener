// @generated automatically by Diesel CLI.

diesel::table! {
    links (id) {
        id -> Int8,
        short_code -> Text,
        target_url -> Text,
        is_active -> Bool,
        created_at -> Timestamptz,
    }
}
