table! {
    users (id) {
        id -> Bpchar,
        name -> Text,
        email -> Varchar,
        password -> Text,
    }
}

table! {
    videos (id) {
        id -> Bpchar,
        user_id -> Bpchar,
        title -> Text,
        description -> Text,
    }
}

joinable!(videos -> users (user_id));

allow_tables_to_appear_in_same_query!(
    users,
    videos,
);
