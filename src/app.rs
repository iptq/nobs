use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use actix_web::{fs, http::Method, App, Error as actix_error};
use failure::{err_msg, Error};
use git2::Repository;
use glob::glob;

use views;
use AppState;
use Config;
use RepoInfo;

pub struct Nobs {
    state: AppState,
}

impl Nobs {
    pub fn from(config: Config) -> Result<Self, Error> {
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
        let mut failed = Vec::new();
        for source in config.sources {
            match app.add_source(&source) {
                Ok(_) => (),
                Err(err) => failed.push(format!("{:?}: {}", source, err)),
            }
        }
        if failed.len() > 1 {
            bail!("Failed to add sources: \n{}", failed.join("\n"))
        }
        Ok(app)
    }
}

impl Nobs {
    fn add_source(&mut self, pattern: &str) -> Result<(), Error> {
        let mut failed = Vec::new();
        for entry in glob(pattern).expect("Bad glob pattern") {
            let result: Result<_, Error> = match &entry {
                Ok(ref path) => {
                    let path = path.canonicalize().unwrap_or(path.clone());
                    let name = path
                        .file_stem()
                        .ok_or(err_msg(format!("File '{:?}' does not have a name.", path)))?
                        .to_str()
                        .ok_or(err_msg(format!(
                            "Could not convert OsStr to str for '{:?}'.",
                            path
                        )))?
                        .to_owned();

                    let _ = Repository::open(&path)?;
                    let info = RepoInfo {
                        name: name.clone(),
                        path,
                    };
                    match self.state.repositories.lock() {
                        Ok(mut repositories) => repositories.insert(name, info),
                        _ => bail!("Could not acquire lock on repositories."),
                    };
                    Ok(())
                }
                Err(err) => bail!("Glob failure: {}", err),
            };
            match result {
                Ok(_) => (),
                Err(err) => failed.push(format!("{:?}: {}", entry, err)),
            }
        }
        if failed.len() > 1 {
            bail!("Failed to add sources: \n{}", failed.join("\n"))
        }
        Ok(())
    }

    pub fn build_app(&self) -> Result<App<AppState>, actix_error> {
        Ok(App::with_state(self.state.clone())
            .handler("/+static", fs::StaticFiles::new("static")?)
            .resource("/", |r| r.method(Method::GET).with(views::host_index))
            .resource("/{repo}/+/{rev}", |r| {
                r.method(Method::GET).with(views::rev_detail)
            })
            .resource("/{repo}", |r| r.method(Method::GET).with(views::repo_index)))
    }
}
