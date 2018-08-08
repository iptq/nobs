use std::path::PathBuf;

use failure::Error;
use git2::Repository;

pub struct Access {
    base_path: PathBuf,
}

impl Access {
    pub fn new(base_path: PathBuf) -> Self {
        Access { base_path }
    }

    pub fn list_repositories(&self) -> Result<Vec<Repository>, Error> {
    	let mut results = Vec::new();
    	Ok(results)
    }
}
