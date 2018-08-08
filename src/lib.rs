extern crate actix_web;
extern crate config as _config;
extern crate failure;
extern crate git2;
extern crate glob;
#[macro_use]
extern crate tera;
#[macro_use]
extern crate serde_json;

mod config;
mod views;

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use actix_web::{fs, http::Method, App};
use failure::Error;
use tera::{Tera,Context};

pub use config::Config;

pub struct Nobs {
    state: AppState,
}

#[derive(Clone)]
pub struct RepoInfo {
    pub path: PathBuf,
}

#[derive(Clone)]
pub struct AppState {
    config: Config,
    pub templates: Arc<Tera>,
    pub repositories: Arc<Mutex<HashMap<String, RepoInfo>>>,
}

impl AppState {
    fn generate_context(&self) -> Context {
        let mut ctx = Context::new();
        ctx.add("site", &json!({
            "title": self.config.title,
        }));
                ctx
    }
}

impl From<Config> for Nobs {
    fn from(config: Config) -> Self {
        let templates = Arc::new(compile_templates!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/templates/**/*"
        )));
        let repositories = Arc::new(Mutex::new(HashMap::new()));
        let state = AppState {
            config: config.clone(),
            templates,
            repositories,
        };

        let mut app = Nobs { state };
        for source in config.sources {
            app.add_source(&source);
        }
        app
    }
}

impl Nobs {
    fn add_source(&mut self, pattern: &str) {
        for entry in glob::glob(pattern).expect("Bad glob pattern") {
            match entry {
                Ok(path) => {
                    let name = path.file_stem().unwrap().to_str().unwrap().to_owned();
                    let info = RepoInfo { path };
                    self.state.repositories.lock().unwrap().insert(name, info);
                }
                Err(_) => (),
            }
        }
    }

    pub fn build_app(&self) -> Result<App<AppState>, Error> {
        Ok(App::with_state(self.state.clone())
            .handler("/+static", fs::StaticFiles::new("static").unwrap())
            .resource("/", |r| r.method(Method::GET).with(views::host_index))
            .resource("/{repo}", |r| r.method(Method::GET).with(views::repo_index)))
    }
}
