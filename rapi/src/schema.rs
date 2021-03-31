table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

table! {
    test_table1 (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
    }
}

table! {
    test_table2 (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
    }
}

allow_tables_to_appear_in_same_query!(
    posts,
    test_table1,
    test_table2,
);
