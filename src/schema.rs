table! {
    repos (id) {
        id -> Int8,
        name -> Varchar,
        active -> Bool,
        user_id -> Int8,
    }
}

table! {
    users (id) {
        id -> Int8,
        username -> Varchar,
        email -> Nullable<Varchar>,
        github_oauth_token -> Nullable<Text>,
        external_id -> Int8,
        site_admin -> Bool,
        installation_id -> Int4,
    }
}

joinable!(repos -> users (user_id));

allow_tables_to_appear_in_same_query!(
    repos,
    users,
);
