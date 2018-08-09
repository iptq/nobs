use actix_web::{error, Error, HttpRequest, HttpResponse, State};

use AppState;

pub fn host_index(
    (req, state): (HttpRequest<AppState>, State<AppState>),
) -> Result<HttpResponse, Error> {
    let mut ctx = state.generate_context(&req);
    ctx.add("title", "Host Index");
    ctx.add(
        "repositories",
        &state
            .repositories
            .lock()
            .unwrap()
            .values()
            .collect::<Vec<_>>(),
    );

    let s = state
        .templates
        .render("host_index.html", &ctx)
        .map_err(|err| {
            eprintln!("Error on template 'repo_index.html': {:?}", err);
            error::ErrorInternalServerError(format!("Template error: {:?}", err))
        })?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}
