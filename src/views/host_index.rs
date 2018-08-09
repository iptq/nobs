use actix_web::{error, Error, HttpRequest, HttpResponse, State};

use AppState;

pub fn host_index(
    (req, state): (HttpRequest<AppState>, State<AppState>),
) -> Result<HttpResponse, Error> {
    let repositories = match state.repositories.lock() {
        Ok(repositories) => repositories
            .values()
            .map(|value| value.clone())
            .collect::<Vec<_>>(),
        _ => return Err(error::ErrorBadRequest("Did not specify a repository.")),
    };

    let mut ctx = state.generate_context(&req);
    ctx.add("title", "Index");
    ctx.add("repositories", &repositories);

    let s = state
        .templates
        .render("host_index.html", &ctx)
        .map_err(|err| {
            eprintln!("Error on template 'repo_index.html': {:?}", err);
            error::ErrorInternalServerError(format!("Template error: {:?}", err))
        })?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}
