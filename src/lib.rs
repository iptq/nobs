extern crate actix_web;
extern crate config as _config;
extern crate failure;
extern crate git2;
extern crate glob;
#[macro_use]
extern crate tera;
#[macro_use]
extern crate serde_json;
extern crate serde;

mod app;
mod config;
mod repo;
mod state;
pub mod views;

pub use app::Nobs;
pub use config::Config;
pub use repo::RepoInfo;
pub use state::AppState;
