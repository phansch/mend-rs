use ::{
    actix::prelude::*,
};
pub mod create_user;
pub mod create_repos;

pub struct GitHubExecutor;
impl Actor for GitHubExecutor {
    type Context = SyncContext<Self>;
}
