use std::{
    fs,
    env,
    path::Path,
    sync::{Arc, Mutex},
};
use serde::Deserialize;
use toml::value;

lazy_static! {
    pub static ref CONFIG: Arc<Mutex<Config>> = Arc::new(Mutex::new(Config::new()));
}

#[derive(Deserialize)]
pub struct Config {
    pub autokommentator: Autokommentator,
    pub kaenguru: Kaenguru,
}

#[derive(Deserialize)]
pub struct Kaenguru {
    pub token: Option<String>,
    pub replies: Vec<Response>
}

#[derive(Deserialize)]
pub struct Autokommentator {
    pub token: Option<String>,
    pub replies: Vec<Response>
}

#[derive(Deserialize)]
pub struct Response {
    pub trigger: value::Array,
    pub response: value::Array,
}

impl Clone for Response {
    fn clone(&self) -> Self {
        Response {
            trigger: self.trigger.to_vec(),
            response: self.response.to_vec()
        }
    }
}

impl Config {
    pub fn new() -> Self {
        // Read in config file location
        let config_file = match env::var("CONFIG_FILE") {
            Ok(o) => o,
            Err(_) => "config.toml".to_string()
        };

        if ! Path::new(&config_file).exists() {
            panic!("Could not locate Configuration file!");
        }

        let config_content = match fs::read_to_string(&config_file) {
            Err(_) => panic!("Could not load configuration file contents!"),
            Ok(s) => s
        };

        let out: Config = toml::from_str(&config_content).unwrap();
        out
    }
}

pub enum Bots {
    Autokommentator,
    KaenguruKnecht,
}
