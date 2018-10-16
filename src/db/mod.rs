#![allow(proc_macro_derive_resolution_fallback)] // This can be removed after diesel-1.4

use ::{
    dotenv::dotenv,
};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel_migrations::*;
use std::env;
use log::{debug, info};

use crate::models::{NewUser, User, Repo, NewRepo};

embed_migrations!("migrations/");
pub fn establish_db_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn = PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url));
    debug!("any_pending? {:?}", diesel_migrations::any_pending_migrations(&conn));
    let mut migration_output = vec![];

    embedded_migrations::run_with_output(&conn, &mut migration_output).unwrap();
    debug!("{:?}", migration_output);
    info!("Migrations should have been executed");

    conn
}

pub fn create_user<'a>(
    conn: &PgConnection,
    username: &'a str,
    email: &'a str,
    external_id: &'a i64,
    installation_id: &'a i32,
) -> User {
    use crate::schema::users;

    let new_user = NewUser {
        username,
        email: Some(email),
        github_oauth_token: None,
        external_id,
        installation_id,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
        .expect("Error saving new User")
}

pub fn create_repos<'a>(
    conn: &PgConnection,
    repos: Vec<NewRepo>,
) -> Vec<Repo> {
    use crate::schema::repos;

    diesel::insert_into(repos::table)
        .values(&repos)
        .get_results(conn)
        .expect("Error saving repos")

}
