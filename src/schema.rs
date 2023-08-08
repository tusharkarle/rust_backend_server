diesel::table! {
    tasks (id) {
        id -> Integer,
        name -> Text,
        description -> Text,
    }
}
