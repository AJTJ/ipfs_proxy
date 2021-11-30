table! {
    api_key (id) {
        id -> Int4,
        user_id -> Int4,
        key_value -> Uuid,
        is_enabled -> Bool,
    }
}

table! {
    key_requests (id) {
        id -> Int4,
        api_key_id -> Int4,
        date_time -> Timestamp,
        successful -> Bool,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Text,
        pw_hash -> Text,
        salt -> Text,
    }
}

joinable!(api_key -> users (user_id));
joinable!(key_requests -> api_key (api_key_id));

allow_tables_to_appear_in_same_query!(
    api_key,
    key_requests,
    users,
);
