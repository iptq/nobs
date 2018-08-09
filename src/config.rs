use _config;
use failure::Error;

#[derive(Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub title: String,
    pub toplevel: String,
    pub recursive: bool,
    pub sources: Vec<String>,
}

impl Config {
    pub fn from_cfg(cfg: &_config::Config) -> Result<Self, Error> {
        let host = cfg.get_str("host")?;
        let port = cfg.get_int("port")? as u16;
        let title = cfg.get_str("title")?;
        let toplevel = cfg.get_str("toplevel")?;
        let recursive = cfg.get_bool("recursive")?;
        let sources = cfg.get_array("sources")?.iter().try_fold(
            Vec::new(),
            |mut it, value| -> Result<_, Error> {
                it.push(value.clone().into_str()?);
                Ok(it)
            },
        )?;
        Ok(Config {
            host,
            port,
            title,
            toplevel,
            recursive,
            sources,
        })
    }
}
