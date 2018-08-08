use actix_web::{error, Error, HttpResponse, State};

use AppState;

pub fn host_index(state: State<AppState>) -> Result<HttpResponse, Error> {
    let mut ctx = state.generate_context();
    ctx.add("title", "Host Index");
    ctx.add(
        "repositories",
        &state
            .repositories
            .lock()
            .unwrap()
            .iter()
            .map(|(name, _repo)| json!({ "name": name }))
            .collect::<Vec<_>>(),
    );
    let s = state
        .templates
        .render("host_index.html", &ctx)
        .map_err(|err| error::ErrorInternalServerError(format!("Template error: {:?}", err)))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}
