use actix_web::{error, Error, HttpRequest, HttpResponse, State};
use git2::ObjectType;

use middleware::RepoFull;
use AppState;
use RevDetails;

pub fn rev_detail(
    (req, repo, state): (HttpRequest<AppState>, RepoFull, State<AppState>),
) -> Result<HttpResponse, Error> {
    let params = req.match_info();
    let rev_name = match params.get("rev") {
        Some(value) => value,
        None => return Err(error::ErrorBadRequest("Did not specify a rev.")),
    };
    let rev = match repo.repo.revparse_single(rev_name) {
        Ok(rev) => rev,
        Err(err) => {
            return Err(error::ErrorBadRequest(format!(
                "Could not parse rev: {}",
                err
            )))
        }
    };
    let mut revwalk = repo.repo.revwalk().unwrap();
    revwalk.push(rev.id()).unwrap();

    // copy their algorithm exactly
    // TODO: figure out how to functionalize this later
    let mut objects = Vec::new();
    let mut cur_id;
    let mut obj = None;
    loop {
        cur_id = match revwalk.next() {
            Some(Ok(value)) => value,
            _ => break,
        };
        let object = match repo.repo.find_object(cur_id, None) {
            Ok(value) => value,
            Err(_) => break,
        };
        objects.push(RevDetails::from(&repo, &rev_name, &object)?);
        obj = Some(object);
        match obj.as_ref().map(|o| o.kind()) {
            Some(Some(ObjectType::Tag)) => (),
            _ => break,
        }
    }
    match obj.map(|o| (o.kind(), o)) {
        Some((Some(ObjectType::Commit), o)) => objects.push(RevDetails::from(
            &repo,
            &rev_name,
            o.peel_to_commit().unwrap().tree().unwrap().as_object(),
        )?),
        _ => (),
    }

    let mut ctx = state.generate_context(&req);
    ctx.add("title", &format!("{} - {}", rev_name, repo.info.name));
    ctx.add("objects", &objects);

    let s = state
        .templates
        .render("rev_detail.html", &ctx)
        .map_err(|err| {
            eprintln!("Error on template 'rev_detail.html': {:?}", err);
            error::ErrorInternalServerError(format!("Template error: {:?}", err))
        })?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}
