#![allow(proc_macro_derive_resolution_fallback)] // This can be removed after diesel-1.4
#![feature(uniform_paths)]
#![feature(proc_macro_non_items)]

// https://github.com/diesel-rs/diesel/issues/1764
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;


pub use log::{info, error};

pub mod auth;
pub mod backend;
pub mod clippy;
pub mod diff;
pub mod frontend;
pub mod lint_report;
pub mod mend_github;
pub mod review_comment;
pub mod handlers;

// Diesel/Database stuff:
pub mod db;
pub mod models;
pub mod schema;
