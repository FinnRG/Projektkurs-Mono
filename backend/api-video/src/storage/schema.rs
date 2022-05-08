table! {
    videos (id) {
        id -> Bpchar,
        title -> Text,
        description -> Nullable<Text>,
        author -> Bpchar,
        date -> Text,
        visibility -> Int4,
        status -> Int4,
    }
}
