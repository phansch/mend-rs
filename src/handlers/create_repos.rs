use ::{
    actix::prelude::*,
    hubcaps::*,
};
use crate::{
    db,
    models,
    handlers::GitHubExecutor,
    mend_github::MendGithub,
};

pub struct CreateRepos {
    pub github_oauth_token: String,
    pub login: String,
    pub user_id: i64,
}
impl Message for CreateRepos {
    type Result = actix_web::Result<Vec<models::Repo>, Error>;
}

impl Handler<CreateRepos> for GitHubExecutor {
    type Result = actix_web::Result<Vec<models::Repo>, Error>;

    /// Fetch the repos from the logged in user and
    /// persist them in our DB
    fn handle(&mut self, msg: CreateRepos, _: &mut Self::Context) -> Self::Result {
        let mut gh = MendGithub::new_from_token(&msg.github_oauth_token);
        let repos = gh.repos(&msg.login);
        let new_repos = repos.iter().map(|r| {
            models::NewRepo {
                name: &r.name,
                active: false,
                user_id: msg.user_id
            }
        }).collect();
        let connection = db::establish_db_connection();
        Ok(db::create_repos(&connection, new_repos))
    }
}

