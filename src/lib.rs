#![feature(uniform_paths)]

pub use log::{error, info};

pub mod clippy;
pub mod diff;
pub mod lint_report;
pub mod mend_github;
pub mod review_comment;
