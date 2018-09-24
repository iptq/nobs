extern crate failure;
extern crate nobs;
extern crate toml;

use std::fs::File;
use std::io::Read;

use failure::Error;
use nobs::{Config, Nobs};

fn main() -> Result<(), Error> {
    let mut file = File::open("nobs.toml")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let config = toml::from_str::<Config>(&contents)?;
    let app = Nobs::from(&config)?;

    app.run();
    Ok(())
}
