#![allow(proc_macro_derive_resolution_fallback)] // This can be removed after diesel-1.4

use std::{
    str::FromStr,
    collections::HashMap,
    env,
};

use ::{
    actix::prelude::*,
    actix::Addr,
    actix_web::http::{header, Method, StatusCode},
    actix_web::middleware::identity::{
        RequestIdentity,
        CookieIdentityPolicy,
        IdentityService
    },
    actix_web::{
        fs, server, App, AsyncResponder, FromRequest, FutureResponse,
        HttpRequest, HttpResponse, Query, Result, test,
    },
    actix_web::test::TestServer,
    diesel::prelude::*,
    futures::Future,
    maud::Markup,
    sentry_actix::SentryMiddleware,
    log::*,
};

use crate::{
    db::establish_db_connection,
    frontend,
    handlers::GitHubExecutor,
    handlers::create_user::CreateUser,
    handlers::create_repos::CreateRepos,
    schema::users::dsl::*,
    models::{User, Repo},
};

// TODO: refactor into lazy_static
// It was somehow tricky to properly set up lazy_static and I don't wanna
// waste that time right now.
pub fn config(key: &str) -> String {
    let mut map = HashMap::new();
    map.insert("mend_hostname", env::var("mend_hostname").unwrap());
    map.insert("mend_gh_client_id", env::var("mend_gh_client_id").unwrap());
    map.insert(
        "mend_gh_client_secret",
        env::var("mend_gh_client_secret").unwrap(),
    );
    map.get(key)
        .expect(&format!("key '{}' not present", key))
        .to_string()
}

fn index(req: &HttpRequest<AppState>) -> Markup {
    let conn = establish_db_connection();
    let fakeuser = User {
        id: 1,
        username: "don't use me".to_string(),
        email: None,
        github_oauth_token: Some("abc".to_string()),
        external_id: 1,
        site_admin: true,
        installation_id: 1,
    };
    if let Some(_) = req.identity() {
        // TODO: find user by decrypted ID in cookie
        frontend::index::render(Some(users.first(&conn).unwrap()))
    } else {
        frontend::index::render(Some(fakeuser))
        // frontend::index::render(None)
    }
}

fn logout(req: HttpRequest<AppState>) -> HttpResponse {
    req.forget();
    HttpResponse::Found()
        .header(header::LOCATION, "/")
        .finish()
}

fn configure(_req: &HttpRequest<AppState>) -> Markup {
    let conn = establish_db_connection();
    let first_user: User = users.first(&conn).expect("first() crashed");
    let repos = Repo::belonging_to(&first_user).get_results(&conn).expect("no results");
    frontend::configure::render(Some(first_user), repos)
}

fn fetch_repos(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    let conn = establish_db_connection();
    let first_user: User = users.first(&conn).expect("first() crashed");
    req.state()
        .gh
        .send(CreateRepos {
            // FIXME: Use currently logged in user
            github_oauth_token: first_user.github_oauth_token.unwrap(),
            login: first_user.username,
            user_id: first_user.id,
        }).from_err()
        .and_then(|res| match res {
            Ok(repos) => Ok(HttpResponse::build(StatusCode::OK)
                            .content_type("text/html; charset=utf-8")
                            .body(&format!("repos: '{:?}'", repos))),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        }).responder()
}

/// Registers a new user with an `installation_id`
///
/// Auth in mend-rs works like this:
///
/// 1. Users create a new mend-rs installation in GitHub
/// 2. They get redirected to `/register`
/// 3. We generate the first JWT and request the user information
/// 4. We store a complete User object in the DB
/// 5. We redirect to `/configure` where users can activate `Repo`s
fn login(req: HttpRequest<AppState>) -> HttpResponse {
    let params = Query::<HashMap<String, String>>::extract(&req).unwrap();
    println!("params: {:?}", params);
    let install_id = params.get("installation_id")
        .expect("installation_id param not provided by GitHub redirect");
    let install_id = i32::from_str(install_id)
        .expect(&format!("Could not parse '{}' to i32", install_id));

    req.state()
        .gh
        .send(CreateUser {
            installation_id: install_id,
        })
        .and_then(|res| match res {
            Ok(_) => Ok(HttpResponse::Found()
                .header(header::LOCATION, "/configure")
                .finish()),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        }).responder();

    HttpResponse::Found()
        .header(header::LOCATION, "/configure")
        .finish()
}


// TODO: Figure out how to parse the different JSON into structs/enums
// TODO: Add unit tests for the different events
enum EventType {
    Installation(String),
}

struct Event {
    event_type: EventType,
    guid: String,
    signature: String,
    user_agent: String,
    // payload: EventPayload,
}

fn event_receiver(req: HttpRequest) -> HttpResponse {
    // TODO:
    // X-GitHub-Event contains the event_type
    info!("req.text(): {:?}", req.headers());
    HttpResponse::Found()
        .header(header::LOCATION, "/")
        .finish()
}

fn p404(_req: &HttpRequest<AppState>) -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("src/backend/static/404.html")?.set_status_code(StatusCode::NOT_FOUND))
}

struct AppState {
    gh: Addr<GitHubExecutor>,
}

pub fn run_server() {
    let sys = actix::System::new("generic_backend");
    let binding = "0.0.0.0:4000";

    let addr = SyncArbiter::start(3, || GitHubExecutor);
    info!("Server listening on {}", binding);
    server::new(|| {
        vec![
            App::new()
                .resource("/event_receiver", |r| r.method(Method::POST).with(event_receiver)),
            App::with_state(AppState { gh: addr.clone() })
                .middleware(SentryMiddleware::builder().emit_header(true).finish())
                // TODO: Add hostname/domain to cookie
                // TODO: Set secure to true in production
                .middleware(IdentityService::new(
                    CookieIdentityPolicy::new(&[0; 32])
                        .name("mend-rs")
                        .path("/")
                        .secure(false)))
                .resource("/", |r| r.f(index))
                .resource("/logout", |r| r.method(Method::GET).with(logout))
                .resource("/configure", |r| r.f(configure))
                .resource("/login", |r| r.method(Method::GET).with(login))
                .resource("/fetch", |r| r.method(Method::GET).with(fetch_repos))
                .default_resource(|r| r.method(Method::GET).f(p404))
        ]
    }).bind(binding)
    .expect("Can not bind to port 4000")
    .start();
    let _ = sys.run();
}

#[test]
fn test_event_receiver() {
    let resp = test::TestRequest::with_header("content-type", "text/plain")
        .run(&event_receiver)
        .unwrap();

    assert_eq!(resp.status(), http::StatusCode::OK);
}
