use actix_web::{error, Error, FromRequest, HttpRequest};
use git2::Repository;

use AppState;
use RepoDetails;
use RepoInfo;

pub struct RepoFull {
    pub info: RepoInfo,
    pub details: RepoDetails,
    pub repo: Repository,
}

impl FromRequest<AppState> for RepoFull {
    type Config = ();
    type Result = Result<RepoFull, Error>;

    fn from_request(req: &HttpRequest<AppState>, _cfg: &Self::Config) -> Self::Result {
        let params = req.match_info();
        let state = req.state();

        let repo_name = match params.get("repo") {
            Some(value) => value,
            None => return Err(error::ErrorBadRequest("Did not specify a repository.")),
        };
        let repo_opt = match state.repositories.lock() {
            Ok(directory) => directory.get(repo_name).cloned(),
            _ => {
                return Err(error::ErrorInternalServerError(
                    "Could not acquire lock on repositories.",
                ))
            }
        };
        let repo_info = match repo_opt {
            Some(value) => value,
            None => return Err(error::ErrorNotFound("Repository not found.")),
        };
        let repo = match Repository::open(&repo_info.path) {
            Ok(repo) => repo,
            Err(err) => {
                return Err(error::ErrorInternalServerError(format!(
                    "Couldn't find repository: {}",
                    err
                )))
            }
        };
        let repo_details = match RepoInfo::get_details_from_repository(&repo) {
            Ok(details) => details,
            Err(err) => {
                return Err(error::ErrorInternalServerError(format!(
                    "Failed to retrieve details: {}",
                    err
                )))
            }
        };

        Ok(RepoFull {
            repo: repo,
            info: repo_info,
            details: repo_details,
        })
    }
}
