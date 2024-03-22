use std::{fs::File, io::Write, path::PathBuf};

use serde::{Deserialize, Serialize};
use serde_yaml;
use thiserror::Error;

use crate::environment;

pub struct Config {
    pub db_file: PathBuf,
    pub covers_directory: PathBuf,
}

impl Config {
    pub fn load() -> Result<Self, ConfigError> {
        // If the config do not exist, create it from the template in the root directory of the project
        // Get the path of the config file and open the file
        let path = Self::config_path()?;
        let config_file = File::open(path).unwrap();

        // Deserialize the config from the yaml file and handle parsing errors
        let configuration: Result<SerializedConfiguration, serde_yaml::Error> =
            serde_yaml::from_reader(config_file);

        if let Err(error) = configuration {
            return Err(ConfigError::Parse(error.to_string()));
        }

        let configuration = configuration.unwrap();

        // Create the config object
        Ok(Self {
            db_file: Self::db_file_path(),
            covers_directory: Self::covers_directory().unwrap(),
        })
    }

    fn config_path() -> Result<PathBuf, ConfigError> {
        let dir = environment::config_directory().join("shadowverse-collection");

        // Create the config directory if it does not exist
        if !dir.exists() {
            match std::fs::create_dir_all(dir.clone()) {
                Ok(_) => {}
                Err(error) => return Err(ConfigError::Create(error.to_string())),
            }
        }

        let file_path = dir.join("config.yaml");

        // Create the config file using the template
        if !file_path.exists() {
            let mut file = File::create(file_path.clone()).unwrap();
            if let Err(error) = file.write_all(include_bytes!("../../config.yaml")) {
                return Err(ConfigError::CreateFile(error.to_string()));
            }
        }

        Ok(file_path)
    }

    fn db_file_path() -> PathBuf {
        environment::config_directory()
            .join("shadowverse-collection")
            .join("shadowverse_utils.db")
    }

    fn covers_directory() -> Result<PathBuf, ConfigError> {
        let covers_dir = environment::local_directory()
            .join("shadowverse-collection")
            .join("covers");

        // Create the config directory if it does not exist
        if !covers_dir.exists() {
            match std::fs::create_dir_all(covers_dir.clone()) {
                Ok(_) => {}
                Err(error) => {
                    return Err(ConfigError::Create(covers_dir.to_str().unwrap().to_owned()))
                }
            }
        }

        Ok(covers_dir)
    }
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("{0}")]
    Parse(String),
    #[error("Could not create the directory {0}")]
    Create(String),
    #[error("Could not create the file {0}")]
    CreateFile(String),
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct SerializedConfiguration {
    shadowverse_api_url: String,
}
