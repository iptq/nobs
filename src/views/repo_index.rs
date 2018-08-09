use actix_web::{error, Error, HttpRequest, HttpResponse, State};

use middleware::RepoFull;
use AppState;

pub fn repo_index(
    (req, repo, state): (HttpRequest<AppState>, RepoFull, State<AppState>),
) -> Result<HttpResponse, Error> {
    let mut ctx = state.generate_context(&req);
    ctx.add("title", &repo.info.name);
    ctx.add("repo", &repo.info);
    ctx.add("details", &repo.details);

    let s = state
        .templates
        .render("repo_index.html", &ctx)
        .map_err(|err| {
            eprintln!("Error on template 'repo_index.html': {:?}", err);
            error::ErrorInternalServerError(format!("Template error: {:?}", err))
        })?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}
