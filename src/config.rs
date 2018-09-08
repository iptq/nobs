use _config;
use failure::Error;

#[derive(Clone)]
pub struct Config {
    pub addr: String,
    pub clone_url: String,
    pub title: String,
    pub toplevel: String,
    pub recursive: bool,
    pub sources: Vec<String>,
}

impl Config {
    pub fn from_cfg(cfg: &_config::Config) -> Result<Self, Error> {
        let addr = cfg.get_str("addr")?;
        let clone_url = cfg.get_str("clone_url")?;
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
            addr,
            clone_url,
            title,
            toplevel,
            recursive,
            sources,
        })
    }
}
