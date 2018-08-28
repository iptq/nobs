use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, Mutex};

use actix_web::{fs, http::Method, App, Error as actix_error};
use failure::{err_msg, Error};
use git2::Repository;
use walkdir::WalkDir;

use humanize::Humanize;
use views;
use AppState;
use Config;
use RepoInfo;

pub struct Nobs {
    state: AppState,
}

#[derive(RustEmbed)]
#[folder = "templates"]
struct Asset;

impl Nobs {
    pub fn from(config: &Config) -> Result<Self, Error> {
        let mut tera = compile_templates!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*"));
        tera.register_filter("humanize_time", |v, _| {
            Ok(<i64>::humanize(&v.as_i64().unwrap()).into())
        });

        let templates = Arc::new(tera);
        let repositories = Arc::new(Mutex::new(HashMap::new()));
        let state = AppState {
            config: config.clone(),
            templates,
            repositories,
        };

        let mut app = Nobs { state };
        let mut failed = Vec::new();
        for source in config.sources.iter() {
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
    fn add_source(&mut self, path: impl AsRef<Path>) -> Result<(), Error> {
        let mut failed = Vec::new();
        let mut iter = WalkDir::new(&path).into_iter();
        loop {
            let entry = match iter.next() {
                None => break,
                Some(Err(err)) => return Err(err_msg(format!("Error walking: {}", err))),
                Some(Ok(entry)) => entry,
            };
            if entry.file_type().is_dir() {
                // TODO: some kind of ignore mechanism
                if entry.file_name() == ".virtualenv" {
                    iter.skip_current_dir();
                    continue;
                }

                let entry_path = entry.path();
                match Repository::open(entry_path) {
                    Ok(_) => {
                        iter.skip_current_dir();
                        let name = entry_path
                            .strip_prefix(&path)
                            .unwrap()
                            .to_str()
                            .unwrap()
                            .to_owned();
                        let info = RepoInfo {
                            name: name.clone(),
                            path: entry_path.to_path_buf(),
                        };
                        match info.get_details() {
                            Err(err) => {
                                failed.push(format!("{:?}: {}", entry_path, err));
                                continue;
                            }
                            _ => (),
                        }
                        match self.state.repositories.lock() {
                            Ok(mut repositories) => repositories.insert(name, info),
                            _ => bail!("Could not acquire lock on repositories."),
                        };
                        continue;
                    }
                    Err(_) => (),
                }
            }
        }
        for entry in failed {
            println!("- {}", entry);
        }
        Ok(())
    }

    pub fn build_app(&self) -> Result<App<AppState>, actix_error> {
        Ok(App::with_state(self.state.clone())
            .handler("/+static", fs::StaticFiles::new("static")?)
            .resource("/", |r| r.method(Method::GET).with(views::host_index))
            .resource("/{repo}/+/{rev}", |r| {
                r.method(Method::GET).with(views::rev_detail)
            }).resource("/{repo}/+log/{rev}", |r| {
                r.method(Method::GET).with(views::log_detail)
            }).resource("/{repo}", |r| r.method(Method::GET).with(views::repo_index)))
    }
}
