use std::fmt;

use hyper::{Body, Response};

#[derive(Debug)]
pub struct Compat(Error);

impl fmt::Display for Compat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ::std::error::Error for Compat {}

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "not found: {}", path)]
    NotFound { path: String },

    #[fail(display = "path missing")]
    PathMissing,
}

impl From<Error> for Compat {
    fn from(err: Error) -> Self {
        Compat(err)
    }
}

impl From<Compat> for Response<Body> {
    fn from(err: Compat) -> Self {
        Response::new(Body::from(format!("{}", err)))
    }
}
