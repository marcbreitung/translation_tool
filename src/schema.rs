table! {
    languages (id) {
        id -> Varchar,
        name -> Varchar,
        lang -> Varchar,
        territory -> Varchar,
    }
}

table! {
    translations (id) {
        id -> Varchar,
        key -> Varchar,
        target -> Text,
        language -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(languages, translations,);
