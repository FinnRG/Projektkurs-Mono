table! {
    comments (id) {
        id -> Int4,
        user_id -> Bpchar,
        video_id -> Bpchar,
        content -> Text,
    }
}

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

joinable!(comments -> videos (video_id));

allow_tables_to_appear_in_same_query!(
    comments,
    users,
    videos,
);
