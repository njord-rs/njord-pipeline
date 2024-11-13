use std::collections::HashMap;

use output::Output;
use serde::Deserialize;
use serde_yaml::Value;
use source::Source;
use task::Task;

pub mod output;
pub mod source;
pub mod step;
pub mod task;

/// Configuration for the data pipeline
#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    /// The list of data sources
    pub sources: Vec<Source>,

    /// The output configuration
    pub outputs: Vec<Output>,

    /// The list of tasks in the data pipeline
    pub tasks: Vec<Task>,

    #[serde(default)]
    pub variables: HashMap<String, Value>,
}

impl Config {
    /// Load the configuration from a file
    ///
    /// # Arguments
    /// * `file_path` - The path to the configuration file
    ///
    /// # Returns
    /// * `Result<Config, Box<dyn std::error::Error>>`
    pub fn from_file(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let config = std::fs::read_to_string(file_path)
            .map_err(|e| format!("Failed to read config file: {}", e))?;

        let config: Config = serde_yaml::from_str(&config)?;

        Ok(config)
    }
}
