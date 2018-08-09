use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Clone)]
pub struct RepoInfo {
    pub name: String,
    pub path: PathBuf,
}

impl RepoInfo {
    fn get_description(&self) -> String {
        let description_file = {
            let mut path = self.path.clone();
            path.push("description");
            path
        };
        let mut description = String::new();
        let mut file = File::open(&description_file).unwrap();
        file.read_to_string(&mut description).unwrap();
        description
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
        state.serialize_field("description", &self.get_description())?;
        state.end()
    }
}
