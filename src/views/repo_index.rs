use actix_web::{error, Error, HttpRequest, HttpResponse, State};

use AppState;

pub fn repo_index(
    (req, state): (HttpRequest<AppState>, State<AppState>),
) -> Result<HttpResponse, Error> {
    let params = req.match_info();
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
    let repo = match repo_opt {
        Some(value) => value,
        None => return Err(error::ErrorNotFound("Repository not found.")),
    };
    let details = match repo.get_details() {
        Ok(details) => details,
        Err(err) => {
            return Err(error::ErrorInternalServerError(format!(
                "Failed to retrieve details: {}",
                err
            )))
        }
    };

    let mut ctx = state.generate_context(&req);
    ctx.add("title", repo_name);
    ctx.add("repo", &repo);
    ctx.add("details", &details);

    let s = state
        .templates
        .render("repo_index.html", &ctx)
        .map_err(|err| {
            eprintln!("Error on template 'repo_index.html': {:?}", err);
            error::ErrorInternalServerError(format!("Template error: {:?}", err))
        })?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}
