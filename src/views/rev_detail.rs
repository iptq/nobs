use actix_web::{error, Error, HttpRequest, HttpResponse, State};

use middleware::RepoFull;
use AppState;

pub fn rev_detail(
    (req, repo, state): (HttpRequest<AppState>, RepoFull, State<AppState>),
) -> Result<HttpResponse, Error> {
    let params = req.match_info();
    let rev_name = match params.get("rev") {
        Some(value) => value,
        None => return Err(error::ErrorBadRequest("Did not specify a rev.")),
    };

    let mut ctx = state.generate_context(&req);
    ctx.add("title", &format!("{} - {}", rev_name, repo.info.name));

    let s = state
        .templates
        .render("rev_detail.html", &ctx)
        .map_err(|err| {
            eprintln!("Error on template 'rev_detail.html': {:?}", err);
            error::ErrorInternalServerError(format!("Template error: {:?}", err))
        })?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}
