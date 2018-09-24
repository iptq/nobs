use futures::{future, Future};
use hyper::{service::Service, Body, Request, Response, StatusCode};

use error::{Compat, Error};
use State;
use TemplateEngine;

pub struct HostIndex {
    state: State,
}

impl Service for HostIndex {
    type ReqBody = Body;
    type ResBody = Body;
    type Error = Compat;
    type Future = Box<Future<Item = Response<Self::ResBody>, Error = Self::Error> + Send>;

    fn call(&mut self, req: Request<Self::ReqBody>) -> Self::Future {
        let repositories = self
            .state
            .get_repositories()
            .values()
            .map(|value| value.clone())
            .collect::<Vec<_>>();

        let mut ctx = self.state.generate_context(&req);
        ctx.add("title", "Index");
        ctx.add("repositories", &repositories);

        let body = match self.state.render("host_index.html", &ctx) {
            Ok(text) => Body::from(text),
            Err(err) => {
                return Box::new(future::err(
                    Error::TemplateRenderFail {
                        inner: format!("{}", err),
                    }.into(),
                ))
            }
        };
        Box::new(future::ok(
            Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "text/html")
                .body(body)
                .unwrap(),
        ))
    }
}

impl HostIndex {
    pub fn new(state: &State) -> Self {
        HostIndex {
            state: state.clone(),
        }
    }
}
