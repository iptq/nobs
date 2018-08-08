use std::path::PathBuf;

use _config;
use failure::Error;

#[derive(Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub base_path: PathBuf,
}

impl Config {
	pub fn from_cfg(cfg: &_config::Config) -> Result<Self, Error> {
		let host = cfg.get_str("host")?;
		let port = cfg.get_int("port")? as u16;
		let base_path = PathBuf::from(cfg.get_str("base_path")?);
		Ok(Config { host, port, base_path })
	}
}
