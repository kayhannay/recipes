table! {
    categories (id) {
        id -> Integer,
        name -> Varchar,
    }
}

table! {
    recipes (id) {
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
    user (uid) {
        uid -> Integer,
        password -> Varchar,
        username -> Varchar,
        name -> Nullable<Varchar>,
        role -> Nullable<Integer>,
        created -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(categories, recipes, user,);
