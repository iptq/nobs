use actix_web::{error, Error, HttpRequest, HttpResponse, State};

use AppState;

pub fn repo_index((req, state): (HttpRequest<AppState>, State<AppState>)) -> Result<HttpResponse, Error> {
    let params = req.match_info();
    let repo_name = params.get("repo").unwrap();
    let mut ctx = state.generate_context();
    ctx.add("title", repo_name);
    let s = state
        .templates
        .render("repo_index.html", &ctx)
        .map_err(|err| error::ErrorInternalServerError(format!("Template error: {:?}", err)))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}
