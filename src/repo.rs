use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use git2::Repository;
use serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Clone)]
pub struct RepoInfo {
    pub name: String,
    pub path: PathBuf,
}

#[derive(Clone)]
pub struct RepoDetails {
    pub description: String,
    pub commits: Vec<CommitDetails>,
}

#[derive(Clone)]
pub struct CommitDetails {}

impl RepoInfo {
    pub fn get_details(&self) -> RepoDetails {
        let repo = Repository::open(&self.path).unwrap();
        let description_file = {
            let mut path = repo.path().to_path_buf().clone();
            path.push("description");
            path
        };
        let mut description = String::new();
        let mut file = File::open(&description_file).unwrap();
        file.read_to_string(&mut description).unwrap();

        let commits = repo
            .revwalk()
            .unwrap()
            .take(5)
            .map(|oid| repo.find_commit(oid).unwrap())
            .map(|commit| CommitDetails {})
            .collect::<Vec<_>>();
        RepoDetails {
            description,
            commits,
        }
    }
}

impl Serialize for RepoInfo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let details = self.get_details();

        let mut state = serializer.serialize_struct("Repository", 3)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("path", &self.path)?;
        state.serialize_field("description", &details.description)?;
        state.end()
    }
}

impl Serialize for RepoDetails {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("RepoDetails", 2)?;
        state.serialize_field("description", &self.description)?;
        state.serialize_field("commits", &self.commits)?;
        state.end()
    }
}

impl Serialize for CommitDetails {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("CommitDetails", 0)?;
        state.end()
    }
}
