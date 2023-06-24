// @generated automatically by Diesel CLI.

diesel::table! {
    categories (id) {
        id -> Int4,
        #[max_length = 50]
        description -> Varchar,
    }
}

diesel::table! {
    chats (id) {
        id -> Uuid,
    }
}

diesel::table! {
    defects (id) {
        id -> Int4,
        text -> Text,
        category_id -> Int4,
    }
}

diesel::joinable!(defects -> categories (category_id));

diesel::allow_tables_to_appear_in_same_query!(
    categories,
    chats,
    defects,
);
