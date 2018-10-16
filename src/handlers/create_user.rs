use std::env;

use ::{
    actix::prelude::*,
    hubcaps::*,
    chrono::Utc,
};
use crate::{
    auth,
    db,
    models,
    handlers::GitHubExecutor,
    mend_github::MendGithub,
};

pub struct CreateUser {
    pub installation_id: i32,
}
impl Message for CreateUser {
    type Result = actix_web::Result<models::User, Error>;
}

fn create_user(installation_id: i32) -> models::User {
    let pkey_path = env::var("pkey_path").unwrap();
    // FIXME: this does not work yet
    let pkey = include_bytes!(pkey_path);
    let jwt = auth::gen_jwt(pkey, Utc::now().timestamp());
    let mut installation_access_token = MendGithub::authenticate_installation(installation_id, &jwt);

    let mut gh = MendGithub::new_from_token(&installation_access_token.token);
    let userinfo = gh.userinfo();
    let connection = db::establish_db_connection();
    db::create_user(
        &connection,
        &userinfo.login,
        &userinfo.email,
        &(userinfo.id as i64),
        &(installation_id as i32)
    )
}

impl Handler<CreateUser> for GitHubExecutor {
    type Result = actix_web::Result<models::User, Error>;

    /// Fetch the userinfo from GitHub and create a new User in our DB
    fn handle(&mut self, msg: CreateUser, _: &mut Self::Context) -> Self::Result {
        Ok(create_user(msg.installation_id))
    }
}
