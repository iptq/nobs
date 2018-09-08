use std::sync::Arc;

use futures::{future, Future};
use hyper::{service::Service, Body, Request, Response, StatusCode};

use app::State;
use error::{Compat, Error};

pub struct HostIndex {
    state: Arc<State>,
}

impl Service for HostIndex {
    type ReqBody = Body;
    type ResBody = Body;
    type Error = Compat;
    type Future = Box<Future<Item = Response<Self::ResBody>, Error = Self::Error> + Send>;

    fn call(&mut self, req: Request<Self::ReqBody>) -> Self::Future {
        let repositories = match self.state.repositories.lock() {
            Ok(repositories) => repositories
                .values()
                .map(|value| value.clone())
                .collect::<Vec<_>>(),
            _ => return Box::new(future::err(Error::PathMissing.into())),
        };

        let mut ctx = self.state.generate_context(&req);
        ctx.add("title", "Index");
        ctx.add("repositories", &repositories);

        let body = match self.state.templates.render("host_index.html", &ctx) {
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
    pub fn new(state: Arc<State>) -> Self {
        HostIndex { state }
    }
}
