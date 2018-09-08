use futures::{future, Future};
use hyper::{service::Service, Body, Request, Response};

use super::statics::Static;
use error::{Compat, Error};

pub struct Parser;

impl Service for Parser {
    type ReqBody = Body;
    type ResBody = Body;
    type Error = Compat;
    type Future = Box<Future<Item = Response<Self::ResBody>, Error = Self::Error> + Send>;

    fn call(&mut self, req: Request<Self::ReqBody>) -> Self::Future {
        let path = match req.uri().path_and_query() {
            Some(pq) => pq.path().to_owned(),
            None => return Box::new(future::err(Error::PathMissing.into())),
        };
        if path.starts_with("/+static") {
            return Box::new(Static::default().call(req).or_else::<_, Box<
                Future<Item = Response<Body>, Error = _> + Send,
            >>(|err| {
                Box::new(future::ok(err.into()))
            }));
        }
        Box::new(future::ok(Response::new(Body::from("nobs"))))
    }
}

impl Default for Parser {
    fn default() -> Self {
        Parser
    }
}
