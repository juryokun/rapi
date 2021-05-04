table! {
    action (id) {
        id -> Int4,
        treatment_id -> Int4,
        name -> Varchar,
    }
}

table! {
    command (id) {
        id -> Int4,
        action_id -> Int4,
        name -> Varchar,
        option -> Nullable<Int4>,
    }
}

table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

table! {
    timing (id) {
        id -> Int4,
        action_id -> Int4,
        name -> Varchar,
    }
}

table! {
    treatment (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    treatment_history (id) {
        id -> Int4,
        action_id -> Int4,
        command_id -> Int4,
        timing_id -> Int4,
        date -> Date,
        option -> Nullable<Int4>,
    }
}

joinable!(action -> treatment (treatment_id));
joinable!(command -> action (action_id));
joinable!(timing -> action (action_id));
joinable!(treatment_history -> action (action_id));
joinable!(treatment_history -> command (command_id));
joinable!(treatment_history -> timing (timing_id));

allow_tables_to_appear_in_same_query!(
    action,
    command,
    posts,
    timing,
    treatment,
    treatment_history,
);
