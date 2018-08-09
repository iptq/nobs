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

mod config;
mod views;

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use actix_web::{fs, http::Method, App, HttpRequest};
use failure::Error;
use git2::Repository;
use serde::ser::{Serialize, SerializeStruct, Serializer};
use tera::{Context, Tera, Value};

pub use config::Config;

pub struct Nobs {
    state: AppState,
}

#[derive(Clone)]
pub struct RepoInfo {
    pub name: String,
    pub path: PathBuf,
    pub description: String,
}

#[derive(Clone)]
pub struct AppState {
    config: Config,
    pub templates: Arc<Tera>,
    pub repositories: Arc<Mutex<HashMap<String, RepoInfo>>>,
}

impl AppState {
    fn generate_breadcrumbs(&self, req: &HttpRequest<Self>) -> Result<Vec<Value>, Error> {
        let path = req.path();
        let parts = path.split("/").collect::<Vec<_>>();

        let mut result = Vec::new();
        match parts.get(1) {
            Some(&"") | None => return Ok(result),
            Some(value) => {
                result.push(json!({ "text": String::from(*value), "url": format!("/{}", value) }))
            }
        };
        Ok(result)
    }
    fn generate_context(&self, req: &HttpRequest<Self>) -> Context {
        let site_metadata = &json!({
            "title": self.config.title,
        });

        let mut ctx = Context::new();
        ctx.add("site", site_metadata);
        match self.generate_breadcrumbs(req) {
            Ok(breadcrumbs) => ctx.add("breadcrumbs", &breadcrumbs),
            _ => (),
        }
        ctx
    }
}

impl Serialize for RepoInfo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Repository", 3)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("path", &self.path)?;
        state.serialize_field("description", &self.description)?;
        state.end()
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
                    let path = path.canonicalize().unwrap_or(path);
                    let name = path.file_stem().unwrap().to_str().unwrap().to_owned();

                    // TODO: don't unwrap
                    let _ = Repository::open(&path).unwrap();
                    let description_file = {
                        let mut path = path.clone();
                        path.push("description");
                        path
                    };
                    let mut description = String::new();
                    let mut file = File::open(&description_file).unwrap();
                    file.read_to_string(&mut description).unwrap();
                    description.trim();

                    let info = RepoInfo {
                        name: name.clone(),
                        path,
                        description,
                    };
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
