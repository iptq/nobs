extern crate config;
extern crate failure;
extern crate nobs;

use std::sync::Arc;

use failure::Error;
use nobs::{Config, Nobs};

fn main() -> Result<(), Error> {
    let mut cfg = config::Config::default();
    cfg.set_default("addr", "127.0.0.1:7700")?;
    cfg.set_default("title", "No-BS Git Viewer")?;
    cfg.set_default("toplevel", "nobs")?;
    cfg.merge(config::File::with_name("nobs"))?;

    let appcfg = Config::from_cfg(&cfg)?;
    let app = Arc::new(Nobs::from(&appcfg)?);

    app.run();
    Ok(())
}
