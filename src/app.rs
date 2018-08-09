use std::sync::{Arc,Mutex};
use std::collections::HashMap;

use actix_web::{App, http::Method, fs};
use failure::Error;
use glob::glob;
use git2::Repository;

use AppState;
use Config;
use RepoInfo;
use views;

pub struct Nobs {
    state: AppState,
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
        for entry in glob(pattern).expect("Bad glob pattern") {
            match entry {
                Ok(path) => {
                    let path = path.canonicalize().unwrap_or(path);
                    let name = path.file_stem().unwrap().to_str().unwrap().to_owned();

                    // TODO: don't unwrap
                    let _ = Repository::open(&path).unwrap();

                    let info = RepoInfo {
                        name: name.clone(),
                        path,
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
