table! {
    categories (cid) {
        cid -> Integer,
        name -> Varchar,
    }
}

table! {
    rezepte (id) {
        id -> Integer,
        name -> Varchar,
        ingredients -> Text,
        preparation -> Text,
        experience -> Nullable<Varchar>,
        time_need -> Nullable<Varchar>,
        number_people -> Nullable<Decimal>,
        created -> Timestamp,
        owner -> Nullable<Decimal>,
        rights -> Nullable<Decimal>,
        category -> Nullable<Integer>,
    }
}

table! {
    sessions (sid) {
        sid -> Varchar,
        ip -> Nullable<Varchar>,
        browser -> Nullable<Varchar>,
        uid -> Integer,
        login_time -> Timestamp,
        expires -> Integer,
    }
}

table! {
    users (uid) {
        uid -> Integer,
        passwd -> Varchar,
        login -> Varchar,
        name -> Nullable<Varchar>,
        role -> Nullable<Integer>,
        created -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    categories,
    rezepte,
    sessions,
    users,
);
