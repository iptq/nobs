use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use tera::{self, Tera};

use Config;
use RepoInfo;
use TemplateEngine;

#[derive(Clone, Default)]
pub struct State {
    inner: Arc<Mutex<InnerState>>,
}

#[derive(Default)]
struct InnerState {
    pub(super) config: Config,
    pub(super) tera: Tera,
    pub(super) repositories: HashMap<String, RepoInfo>,
}

impl InnerState {
    pub fn set_config(&mut self, config: &Config) {
        self.config = config.clone();
    }
}

impl State {
    pub fn set_config(&mut self, config: &Config) {
        let mut inner = self.inner.lock().unwrap();
        inner.set_config(config);
    }
    pub fn get_config(&self) -> Config {
        let inner = self.inner.lock().unwrap();
        inner.config.clone()
    }
    pub fn add_repository(&self, name: String, info: RepoInfo) {
        let mut inner = self.inner.lock().unwrap();
        inner.repositories.insert(name, info);
    }
    pub fn get_repositories(&self) -> HashMap<String, RepoInfo> {
        let inner = self.inner.lock().unwrap();
        inner.repositories.clone()
    }
}

impl TemplateEngine for State {
    type Template = (String, String);
    type Context = tera::Context;
    type Error = tera::Error;

    fn add_templates(&mut self, templates: impl Iterator<Item = Self::Template>) {
        let templates = templates.collect::<Vec<_>>();
        let mut inner = self.inner.lock().unwrap();
        inner.tera.add_raw_templates(
            templates
                .iter()
                .map(|(a, b)| (a.as_ref(), b.as_ref()))
                .collect::<Vec<_>>(),
        ).expect("could not add templates");
    }

    fn render(
        &self,
        template: impl AsRef<str>,
        context: &Self::Context,
    ) -> Result<String, Self::Error> {
        let inner = self.inner.lock().unwrap();
        inner.tera.render(template.as_ref(), &context)
    }
}
