table! {
    comments (id) {
        id -> Int4,
        user_id -> Bpchar,
        video_id -> Bpchar,
        content -> Text,
    }
}

table! {
    likes (user_id, video_id) {
        value -> Bool,
        user_id -> Bpchar,
        video_id -> Bpchar,
    }
}

table! {
    tags (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Text>,
        author -> Bpchar,
        deleted -> Bool,
    }
}

table! {
    tag_to_video (tag_id, video_id) {
        tag_id -> Int4,
        video_id -> Bpchar,
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

joinable!(tag_to_video -> tags (tag_id));
joinable!(tags -> users (author));
joinable!(comments -> users (user_id));

allow_tables_to_appear_in_same_query!(comments, likes, tags, tag_to_video, users, videos,);
