extern crate actix_web;
extern crate config as _config;
extern crate failure;
extern crate git2;
#[macro_use]
extern crate rust_embed;
#[macro_use]
extern crate tera;

mod access;
mod config;
mod views;

use actix_web::{http::Method, App, fs};
use failure::Error;
use tera::Tera;

pub use access::Access;
pub use config::Config;

pub struct Nobs {
    access: Access,
    config: Config,
}

pub struct AppState {
    pub templates: Tera,
}

impl From<Config> for Nobs {
    fn from(config: Config) -> Self {
        let access = Access::new(config.base_path.clone());
        Nobs { access, config }
    }
}

impl Nobs {
    pub fn build_app(&self) -> Result<App<AppState>, Error> {
        let tera = compile_templates!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*"));
        Ok(App::with_state(AppState { templates: tera })
            .handler("/+static", fs::StaticFiles::new("static").unwrap())
            .resource("/", |r| r.method(Method::GET).with(views::Index)))
    }
}
