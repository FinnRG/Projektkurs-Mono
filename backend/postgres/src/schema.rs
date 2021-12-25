table! {
    users (id) {
        name -> Text,
        email -> Varchar,
        password -> Text,
        id -> Bpchar,
    }
}

table! {
    videos (id) {
        id -> Bpchar,
        title -> Text,
        description -> Nullable<Text>,
        creation_date -> Date,
        user_id -> Nullable<Bpchar>,
    }
}

joinable!(videos -> users (user_id));

allow_tables_to_appear_in_same_query!(
    users,
    videos,
);
