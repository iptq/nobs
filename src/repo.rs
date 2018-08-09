use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use failure::{err_msg, Error};
use git2::{Branch, BranchType, Repository};
use serde::ser::{self, Serialize, SerializeStruct, Serializer};

#[derive(Clone)]
pub struct RepoInfo {
    pub name: String,
    pub path: PathBuf,
}

#[derive(Clone)]
pub struct RepoDetails {
    pub branches: Vec<String>,
    pub description: String,
    pub commits: Vec<CommitDetails>,
}

#[derive(Clone)]
pub struct CommitDetails {
    pub hash: String,
    pub author: String,
    pub summary: String,
}

impl RepoInfo {
    pub fn get_details(&self) -> Result<RepoDetails, Error> {
        let repo = Repository::open(&self.path)?;
        // get description
        let description_file = {
            let mut path = repo.path().to_path_buf().clone();
            path.push("description");
            path
        };
        let mut description = String::new();
        let mut file = File::open(&description_file)?;
        file.read_to_string(&mut description)?;

        // get branches and commits
        let mut revwalk = repo.revwalk()?;
        let mut branches = repo.branches(None)?;
        let branch: Branch = repo.find_branch("master", BranchType::Local).or_else(
            |_| -> Result<_, Error> {
                Ok(branches
                    .next()
                    .ok_or(err_msg("No branches exist in this repo."))??
                    .0)
            },
        )?;
        let commit = branch.get().peel_to_commit()?;
        revwalk.push(commit.id())?;
        let commits = revwalk
            .take(5)
            .filter_map(|oid| match oid {
                Ok(oid) => match repo.find_commit(oid) {
                    Ok(value) => Some(value),
                    Err(_) => None,
                },
                _ => None,
            })
            .try_fold(Vec::new(), |mut it, commit| -> Result<_, Error> {
                it.push(CommitDetails {
                    hash: format!("{}", commit.id()),
                    author: commit.author().name().unwrap_or("").to_owned(),
                    summary: commit.summary().unwrap_or("").to_owned(),
                });
                Ok(it)
            })?;
        let branches = repo
            .branches(None)?
            .map(|branch| -> Result<_, Error> {
                let (branch, branch_type) = branch?;
                match branch_type {
                    BranchType::Local => Ok(branch.name()?.ok_or(err_msg("Not UTF-8"))?.to_owned()),
                    _ => Err(err_msg("Not a local branch.")),
                }
            })
            .filter_map(Result::ok)
            .collect::<Vec<_>>();
        Ok(RepoDetails {
            branches,
            description,
            commits,
        })
    }
}

impl Serialize for RepoInfo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let details = match self.get_details() {
            Ok(details) => details,
            Err(err) => {
                return Err(ser::Error::custom(format!(
                    "Failed to retrieve details: {}",
                    err
                )))
            }
        };

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
        let mut state = serializer.serialize_struct("RepoDetails", 3)?;
        state.serialize_field("branches", &self.branches)?;
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
        let mut state = serializer.serialize_struct("CommitDetails", 2)?;
        state.serialize_field("hash", &self.hash)?;
        state.serialize_field("author", &self.author)?;
        state.serialize_field("summary", &self.summary)?;
        state.end()
    }
}
