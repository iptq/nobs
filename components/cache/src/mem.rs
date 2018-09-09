use std::collections::HashMap;

use Cache;

pub struct MemCache {
    inner: HashMap<String, String>,
}

impl Cache for MemCache {
    fn set(&mut self, key: &str, value: impl AsRef<str>) {
        self.inner.insert(key.to_owned(), value.as_ref().to_owned());
    }
    fn get(&self, key: &str) -> &str {
        self.inner
            .get(key)
            .map(|s| s.as_ref())
            .unwrap_or_else(|| "")
    }
}
