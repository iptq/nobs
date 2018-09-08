use futures::{future, Future};
use hyper::{service::Service, Body, Request, Response, StatusCode};
use mime_guess::guess_mime_type;

use error::{Compat, Error};

#[derive(RustEmbed)]
#[folder = "static"]
struct StaticFS;

pub struct Static;

impl Service for Static {
    type ReqBody = Body;
    type ResBody = Body;
    type Error = Compat;
    type Future = Box<Future<Item = Response<Self::ResBody>, Error = Self::Error> + Send>;

    fn call(&mut self, req: Request<Self::ReqBody>) -> Self::Future {
        let path = match req.uri().path_and_query() {
            Some(pq) => pq.path().to_owned(),
            None => return Box::new(future::err(Error::PathMissing.into())),
        };
        let path = path.trim_left_matches("/+static/").to_owned();
        let mime = guess_mime_type(&path);

        match StaticFS::get(&path) {
            Some(asset) => Box::new(future::ok(
                Response::builder()
                    .status(StatusCode::OK)
                    .header("Content-Type", format!("{}", mime))
                    .body(Body::from(asset))
                    .unwrap(),
            )),
            None => Box::new(future::err(Error::NotFound { path }.into())),
        }
    }
}

impl Default for Static {
    fn default() -> Self {
        Static
    }
}
