use std::{
    fs,
    env,
    path::Path,
};
use serde::{Serialize, Deserialize};
use toml::value;

lazy_static!(
    static ref CONFIG: Config = Config::new();
);

#[derive(Deserialize)]
pub struct Config {
    token: Token,
    kaenguru: Vec<Response>,
    autokommentator: Vec<Response>,
}

#[derive(Deserialize)]
pub struct Response {
    trigger: Option<String>,
    trigger_pool: Option<value::Array>,
    response: Option<String>,
    response_pool: Option<value::Array>,
}

#[derive(Deserialize)]
pub struct Token {
    autokommentator: Option<String>,
    kaenguru: Option<String>,
}

impl Config {
    pub fn new() -> Self {
        // Read in config file location
        let config_file = match env::var("CONFIG_FILE") {
            Ok(o) => o,
            Err(a) => "config.toml".to_string()
        };

        if ! Path::new(&config_file).exists() {
            panic!("Could not locate Configuration file!");
        }

        let config_content = match fs::read_to_string(&config_file) {
            Err(why) => panic!("Could not load configuration file contents!"),
            Ok(s) => s
        };

        let out: Config = toml::from_str(&config_content).unwrap();
        out
    }
}

