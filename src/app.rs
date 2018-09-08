use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, Mutex};

use failure::{err_msg, Compat, Error};
use futures::{future, Future};
use git2::Repository;
use hyper::{self, Server};
use tera::Tera;
use walkdir::WalkDir;

use humanize::Humanize;
use services::parser::Parser;
use Config;
use RepoInfo;

pub struct Nobs {
    pub config: Config,
    pub templates: Arc<Tera>,
    pub repositories: Arc<Mutex<HashMap<String, RepoInfo>>>,
}

#[derive(RustEmbed)]
#[folder = "templates"]
struct Templates;

impl Nobs {
    pub fn from(config: &Config) -> Result<Self, Error> {
        let mut tera = Tera::default();
        for item in Templates::keys() {
            println!("loading item {}", &item);
            let asset = Templates::get(item).unwrap();
            let template = String::from_utf8(asset).unwrap();
            tera.add_raw_template(item, template.as_ref()).unwrap();
        }

        tera.register_filter("humanize_time", |v, _| {
            Ok(<i64>::humanize(&v.as_i64().unwrap()).into())
        });

        let templates = Arc::new(tera);
        let repositories = Arc::new(Mutex::new(HashMap::new()));

        let mut app = Nobs {
            config: config.clone(),
            templates,
            repositories,
        };
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

// fn static_handler(req: &HttpRequest<AppState>) -> Result<HttpResponse, Error> {
//     let path = &req.path()["/+static/".len()..];
//     let mime = guess_mime_type(path);
//     Ok(match Static::get(path) {
//         Some(content) => HttpResponse::Ok().content_type(mime.as_ref()).body(content),
//         None => HttpResponse::NotFound().body("404 Not Found"),
//     })
// }

impl Nobs {
    fn add_source(&mut self, path: impl AsRef<Path>) -> Result<(), Error> {
        let mut failed = Vec::new();
        let mut iter = WalkDir::new(&path).follow_links(true).into_iter();
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
                        match self.repositories.lock() {
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

    pub fn run(&self) {
        let addr = self.config.addr.parse().unwrap();
        let server =
            Server::bind(&addr).serve(|| future::ok::<_, Compat<Error>>(Parser::default()));
        hyper::rt::run(server.map_err(|_| ()));
        // Ok(App::with_state(self.state.clone())
        //     .handler("/+static", static_handler)
        //     .resource("/", |r| r.method(Method::GET).with(views::host_index))
        //     .resource("/{repo}/+/{rev}", |r| {
        //         r.method(Method::GET).with(views::rev_detail)
        //     }).resource("/{repo}/+log/{rev}", |r| {
        //         r.method(Method::GET).with(views::log_detail)
        //     }).resource("/{repo}", |r| r.method(Method::GET).with(views::repo_index)))
    }
}
