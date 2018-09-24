#[derive(Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "Config::default_addr")]
    addr: String,
    #[serde(default)]
    base_url: Option<String>,
    #[serde(default = "Config::default_title")]
    title: String,
    #[serde(default)]
    sources: Vec<String>,
    #[serde(default)]
    ignores: Vec<String>,
}

impl Config {
    pub fn get_addr(&self) -> &str {
        &self.addr
    }
    pub fn get_title(&self) -> &str {
        &self.title
    }
    pub fn get_sources(&self) -> ::std::slice::Iter<String> {
        self.sources.iter()
    }

    pub fn default_addr() -> String {
        "127.0.0.1:7700".to_owned()
    }
    pub fn default_title() -> String {
        "NOBS Static Git Viewer".to_owned()
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            addr: "127.0.0.1:7700".to_string(),
            base_url: None,
            title: "NOBS Static Git Viewer".to_string(),
            sources: Vec::new(),
            ignores: Vec::new(),
        }
    }
}
