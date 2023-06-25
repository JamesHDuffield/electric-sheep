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

diesel::table! {
    messages (id) {
        id -> Int4,
        #[max_length = 12]
        role -> Varchar,
        content -> Text,
        created_at -> Timestamp,
        chat_id -> Uuid,
    }
}

diesel::joinable!(defects -> categories (category_id));
diesel::joinable!(messages -> chats (chat_id));

diesel::allow_tables_to_appear_in_same_query!(
    categories,
    chats,
    defects,
    messages,
);
