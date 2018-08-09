use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use actix_web::{Error, HttpRequest};
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
        result.push(json!({ "text": "nobs", "url": "/" }));
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
    pub fn generate_context(&self, req: &HttpRequest<Self>) -> Context {
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
