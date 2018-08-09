use actix_web::{error, Error, HttpRequest, HttpResponse, State};

use AppState;

pub fn repo_index(
    (req, state): (HttpRequest<AppState>, State<AppState>),
) -> Result<HttpResponse, Error> {
    let params = req.match_info();
    let repo_name = params.get("repo").unwrap();
    let repo_opt = {
        let directory = state.repositories.lock().unwrap();
        directory.get(repo_name).cloned()
    };
    let repo = match repo_opt {
        Some(value) => value,
        None => return Err(error::ErrorNotFound("Repository not found.")),
    };

    let mut ctx = state.generate_context(&req);
    ctx.add("title", repo_name);
    ctx.add("repo", &repo);

    let s = state
        .templates
        .render("repo_index.html", &ctx)
        .map_err(|err| {
            eprintln!("Error on template 'repo_index.html': {:?}", err);
            error::ErrorInternalServerError(format!("Template error: {:?}", err))
        })?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}
