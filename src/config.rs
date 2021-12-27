use std::{
    fs::{self, File},
    env,
    path::Path,
    io::Write,
    process::exit,
    sync::{Arc, Mutex},
};
use regex::Regex;
use serde::Deserialize;
use toml::value;

/// The response that is injected into a panic, if the config file was configured falsely
const PANIC_RESPONSE: &str = "Please create a config file yourself or try setting the environment CONFIG_FILE to valid file location!";

/// The versions of the config this program is compatible with
const COMPATIBLE_VERSIONS: [(u32, u32); 1] = [
    (0, 2),
];

lazy_static! {
    ///
    /// The global, thread safe configuration of all of this bot.
    ///
    /// Examples:
    /// ```
    /// let config_arc = Arc::clone(&CONFIG);
    /// let mut config_lock = config_arc.lock();
    /// let value = match config_lock {
    ///     Ok(config) => {
    ///         // ACCESS CONFIG FIELDS HERE AND COPY THEM INTO VALUE
    ///         // Example: String::from(&config.autokommentator.token)
    ///     },
    ///     Err(why) => {
    ///         panic!("Something went wrong internally: {:?}\nMutex is poisoned: {}", why, why);
    ///     }
    /// };
    /// ```
    ///
    pub static ref CONFIG: Arc<Mutex<Config>> = Arc::new(Mutex::new(Config::new()));
}

#[derive(Deserialize)]
/// The default configuration struct that holds the global configuration structure
pub struct Config {
    /// The version this config file was created with
    pub version: String,
    /// Holds configuration for the Autokommentator bot
    pub autokommentator: Autokommentator,
    /// Holds configuration for the Känguru Knecht bot
    pub kaenguru: Kaenguru,
}

#[derive(Deserialize)]
/// Structures the data used by the Känguru Knecht bot
pub struct Kaenguru {
    /// The token that is used to log into discord
    pub token: Option<String>,
    /// The replies and messages that this bot should react to.
    pub responses: Vec<Response>
}

#[derive(Deserialize)]
/// Structures the data used by the Autkommentator bot
pub struct Autokommentator {
    /// The token that is used to log into discord
    pub token: Option<String>,
    /// The replies and messages that this bot should react to.
    pub responses: Vec<Response>
}

#[derive(Deserialize)]
/// Structures the data on how to react to messages
pub struct Response {
    /// A list of strings that trigger a reaction in a message
    pub trigger: value::Array,
    /// A list of strings that are replied to the message:
    /// If there are multiple elements in this list, one is selected randomly.
    pub response_pool: value::Array,
}

impl Clone for Response {
    fn clone(&self) -> Self {
        Response {
            trigger: self.trigger.to_vec(),
            response_pool: self.response_pool.to_vec()
        }
    }
}

impl Config {
    /// Creates a configuration object from a config file.
    ///
    /// Gets the configuration file location from the environment `CONFIG_FILE`,
    /// or if it was not set, defaults to `config.toml`.
    ///
    /// **This function should be only called once when the program is started, as
    /// always reading the file in again takes a lot of time.**
    pub fn new() -> Self {
        // Read in config file location
        let config_file = match env::var("CONFIG_FILE") {
            Ok(o) => o,
            Err(_) => "config.toml".to_string()
        };

        if ! Path::new(&config_file).exists() {
            eprintln!("Could not locate Configuration file! Using defaults!");

            make_default_config(&config_file);
        }

        let config_content = match fs::read_to_string(&config_file) {
            Err(_) => panic!("Could not load configuration file contents!"),
            Ok(s) => s
        };

        {
            let (compatible, version) = check_version(&config_content);
            if ! compatible {
                eprintln!("The config file version ({}) is not compatible with your program version ({}.{})!\nPlease inspect the Changelog (https://github.com/MaFeLP/discord_bots/releases) and see how to change the config file accordingly!",
                          version,
                          env!("CARGO_PKG_VERSION_MAJOR"),
                          env!("CARGO_PKG_VERSION_MINOR")
                );
                exit(1);
            }
        }

        let out: Config = match toml::from_str(&config_content) {
            Err(_) => {
                let example_config_file = format!("{}.example", config_file);
                make_default_config(&example_config_file);
                eprintln!("Configuration file invalid!\nAn example can be found here: {}", example_config_file);
                exit(1);
            },
            Ok(config) => config,
        };
        out
    }
}

/// A function to make a configuration file.
///
/// # Arguments
///
/// * `config_file`: The location where the new config file should be placed.
///
/// returns: ()
///
/// # Examples
///
/// ```
/// let config_file: String = String::from("config.toml");
/// let future = make_default_config(&config_file);
/// block_on(future);
/// ```
fn make_default_config(config_file: &String) {
    // Try to create the file
    let mut file = match File::create(config_file) {
        Ok(f) => f,
        Err(why) => panic!("Could not create the config file: {:?}.\n{}\n{}", why, PANIC_RESPONSE, why)
    };

    // Write the config file
    match writeln!(&mut file, "# Config created automatically\n{}", include_str!("../config.toml.example")) {
        Ok(_) => println!("Written default configuration to {}", config_file),
        Err(why) => panic!("Could not write default configuration file to {}: {:?}\n{}\n{}", config_file, why, PANIC_RESPONSE, why)
    };
}

/// Checks if the inputted config is compatible with the program
///
/// # Arguments
///
/// * `config_content`: The content of the config file to check for compatibility
///
/// returns: (bool, String)
///
/// # Examples
///
/// ```
/// if ! check_version("config.toml")[0] {
///     panic!("Config version incompatible!");
/// }
/// ```
fn check_version(config_content: &String) -> (bool, String) {
    // Get the version, by looping over the config contents, searching for the config line and
    // getting the version part of it.
    let version = {
        let version_line = Regex::new("(?m)^version *= *\"(?P<major>\\d*)\\.(?P<minor>\\d*)\" *(|#.*)$").unwrap();
        let captures = version_line.captures(config_content).unwrap();
        let out = format!("{}.{}", captures.name("major").unwrap().as_str(), captures.name("minor").unwrap().as_str());
        dbg!("{}", &out);
        out
    };
    // Config file is always compatible with its associated program version
    if format!("{}.{}", env!("CARGO_PKG_VERSION_MAJOR"), env!("CARGO_PKG_VERSION_MINOR")).eq_ignore_ascii_case(&version) {
        return (true, version);
    }
    // If config file version is not the program version, check if it is still compatible:
    for (major, minor) in COMPATIBLE_VERSIONS {
        if format!("{}.{}", major, minor).eq_ignore_ascii_case(&version) {
            return (true, version);
        }
    }
    return (false, version);
}

///
/// An enum that represents all bots in this project. This enables non bot-specific functions
/// (For example [crate::replies::reply_to]) to behave differently based on which bot is used.
///
pub enum Bots {
    /// Represents the Autokommentator bot found in [xd.rs](crate::xd)
    Autokommentator,

    /// Represents the Känguru Knecht bot found in [kaenguru.rs](crate::kaenguru)
    KaenguruKnecht,
}
