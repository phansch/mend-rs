#![feature(uniform_paths)]

mod clippy;
mod diff;
mod lint_report;
mod mend_github;
mod review_comment;

use std::env;
use std::io;

use crate::diff::Diff;
use crate::mend_github::MendGithub;
use crate::review_comment::ReviewComment;
use faktory::ConsumerBuilder;

fn main() {
    init_sentry();
    sentry::integrations::env_logger::init(None, Default::default());
    start_faktory_consumer();
}

fn start_faktory_consumer() {
    println!(
        "Starting Faktory consumer on {}",
        env::var("FAKTORY_URL").unwrap()
    );
    let mut c = ConsumerBuilder::default();
    c.register("pull_request", |job| -> io::Result<()> {
        println!("received job: {:?}", job);
        analyze_repo().unwrap();
        Ok(())
    });
    let mut c = c.connect(None).unwrap();
    if let Err(e) = c.run(&["default"]) {
        println!("worker failed: {}", e);
    }
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
    if let Ok(sentry_url) = env::var("SENTRY_URL") {
        let _guard = sentry::init(sentry_url);

        env::set_var("RUST_BACKTRACE", "1");

        sentry::integrations::panic::register_panic_handler();
    }
}
