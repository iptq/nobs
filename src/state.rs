use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use tera::{Context, Tera, Value};

use Config;
use RepoInfo;

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub templates: Arc<Tera>,
    pub repositories: Arc<Mutex<HashMap<String, RepoInfo>>>,
}

impl AppState {
    fn generate_breadcrumbs(&self, req: &HttpRequest<Self>) -> Result<Vec<Value>, Error> {
        let path = req.path();
        let parts = path.split("/").collect::<Vec<_>>();

        let mut result = Vec::new();
        let repo;
        result.push(json!({ "text": self.config.toplevel, "url": "/" }));
        match parts.get(1) {
            Some(&"") | None => return Ok(result),
            Some(value) => {
                repo = String::from(*value);
                result.push(json!({ "text": String::from(*value), "url": format!("/{}", value) }))
            }
        };
        match parts.get(3) {
            Some(&"") | None => return Ok(result),
            Some(value) => {
                result.push(
                    json!({ "text": String::from(*value), "url": format!("/{}/+/{}", repo, value) }),
                );
            }
        };
        Ok(result)
    }
}
