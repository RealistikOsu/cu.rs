use serde::{Serialize, Deserialize};
use std::fs::File;
use std::path::Path;
use std::io::{Write, Read};

const CONFIG_DIR: &str = "config.json";

/// # Config
/// The class storing the kisumi.rs config values.
#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub http_sock: String,
    pub server_name: String,
    pub server_bot_id: i32,
    pub sql_server: String,
    pub sql_db: String,
    pub sql_user: String,
    pub sql_passwd: String,
}

impl Config {
    /// Creates an instance of `Config` featuring default values. Meant for
    /// new config generation.
    pub fn default() -> Self {
        Self {
            http_sock: "/tmp/kisumi.sock".to_string(),
            server_name: "KisumiDev".to_string(),
            server_bot_id: 999,
            sql_server: "localhost".to_string(),
            sql_db: "rosu".to_string(),
            sql_user: "rosu".to_string(),
            sql_passwd: "".to_string(),
        }
    }

    /// # Config Write String.
    /// Writes the config to a JSON formatted string.
    #[inline(always)]
    pub fn write_string(&self) -> String {
        // Panic here is fine as this can only go wrong if the user is 
        // retarded on startup.
        serde_json::to_string_pretty(&self).unwrap()
    }

    /// # Config Write File.
    /// Writes the current config to the file `crate::CONFIG_NAME`.
    /// ## Args
    /// - file_exists (`bool`): Whether the empty file already exists for the
    ///     config. If set to false, a new file is created.
    pub fn write_file(&self, file_exists: bool) {
        let mut f = if !file_exists {
            File::create(CONFIG_DIR).unwrap()
        } else {
            File::open(CONFIG_DIR).unwrap()
        };

        f.write(self.write_string().as_bytes()).expect("Could not write config file..."); // Sucks to suck if err.
    }

    /// # Config Load
    /// Loads the config from the configuration file `crate::CONFIG_NAME` and 
    /// creates an instance of `Config` from the data inside it.
    pub fn from_file() -> Self {
        let mut f = File::open(CONFIG_DIR).unwrap();
        let mut s = String::with_capacity(250);
        f.read_to_string(&mut s).expect("Could not read config.");

        serde_json::from_str(&s).unwrap()
    }

    /// Checks if there is already a config stored.
    #[inline(always)]
    pub fn config_exists() -> bool {
        Path::new(CONFIG_DIR).exists()
    }
}
