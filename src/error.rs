use std::fmt;

use hyper::{Body, Response, StatusCode};

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
    #[fail(display = "resource not found: {}", path)]
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
        let status = match err {
            Compat(Error::NotFound { .. }) => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        Response::builder()
            .status(status)
            .body(Body::from(format!("{}", err)))
            .unwrap()
    }
}
