use crate::schema::{users, repos};
use diesel::{Associations, Identifiable, Insertable, Queryable};
use serde_derive::Deserialize;

#[derive(Identifiable, Queryable, Debug)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: Option<String>,
    pub github_oauth_token: Option<String>,
    pub external_id: i64,
    pub site_admin: bool,
    pub installation_id: i32,
}

#[derive(Insertable, Debug)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub email: Option<&'a str>,
    pub github_oauth_token: Option<&'a str>,
    pub external_id: &'a i64,
    pub installation_id: &'a i32,
}

#[derive(Identifiable, Queryable, Deserialize, Associations, PartialEq, Debug)]
#[belongs_to(User)]
pub struct Repo {
    pub id: i64,
    pub name: String,
    /// Wether or not mend-rs should run on this Repo
    pub active: bool,
    /// The `User` this repo belongs to
    pub user_id: i64,
}

#[derive(Insertable, Deserialize, Debug)]
#[table_name = "repos"]
pub struct NewRepo<'a> {
    pub name: &'a str,
    pub active: bool,
    pub user_id: i64,
}
