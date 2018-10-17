#![allow(proc_macro_derive_resolution_fallback)] // This can be removed after diesel-1.4
#![feature(uniform_paths)]
#![feature(proc_macro_non_items)]
#![feature(proc_macro_hygiene)]

// https://github.com/diesel-rs/diesel/issues/1764
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

mod auth;
mod backend;
mod clippy;
mod db;
mod diff;
mod frontend;
mod lint_report;
mod mend_github;
mod models;
mod review_comment;
mod schema;
mod handlers;

use std::env;

use crate::diff::Diff;
use crate::mend_github::MendGithub;
use crate::review_comment::ReviewComment;

fn main() {
    init_sentry();
    sentry::integrations::env_logger::init(None, Default::default());
    backend::run_server();
    // analyze_repo().unwrap();
}

fn analyze_repo() -> Result<(), String> {
    // NOTE: this is just a test/example of how everything would work together.  I imagine this
    // method will be scrapped once it's figured out how and where this code is executed.
    let user = "phansch";
    let repo = "rust-worksheets";
    let mut github = MendGithub::new_from_token("SUPER_SECRET_TOKEN_HERE");
    let diff = Diff::from_pr(&mut github, user, repo, 10).unwrap();

    let path = github.download_archive(user, repo, "basic_pdf_generation");

    let clippy_output = clippy::run(&path);
    let lint_report = lint_report::parse_json(&clippy_output);
    let comments_to_post = ReviewComment::from_lint_report(&lint_report, &diff);
    github.post_comments(&comments_to_post, user, repo);
    Ok(())
}

fn init_sentry() {
    let _guard = sentry::init(env::get("SENTRY_URL"));

    env::set_var("RUST_BACKTRACE", "1");

    sentry::integrations::panic::register_panic_handler();
}
