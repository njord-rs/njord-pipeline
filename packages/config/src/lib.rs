use serde::Deserialize;

/// Configuration for the data pipeline
#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    /// The list of data sources
    pub sources: Vec<Source>,

    /// The output configuration
    pub outputs: Vec<Output>,

    /// The list of tasks in the data pipeline
    pub tasks: Vec<Task>,
}

/// Represents a single data source
#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum Source {
    /// HTTP source
    #[serde(rename = "http")]
    Http {
        url: String,
        #[serde(default = "default_format")]
        format: String,
        name: String,
    },

    /// File source
    #[serde(rename = "file")]
    File { path: String, name: String },
}

/// Output configuration
#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum Output {
    #[serde(rename = "file")]
    File {
        path: String,
        #[serde(default = "default_format")]
        format: String,
    },
}

impl Output {
    /// Returns the name of the output
    pub fn name(&self) -> &str {
        match self {
            Output::File { .. } => "file",
        }
    }
}

/// The different steps in the data pipeline
#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum Step {
    #[serde(rename = "json_query")]
    JsonQuery { query: String },
    #[serde(rename = "math")]
    Math { operation: String },
    #[serde(rename = "convert")]
    Convert { from: String },
}

/// The different tasks
#[derive(Debug, Deserialize, Clone)]
pub struct Task {
    pub name: String,
    pub source: String,
    pub steps: Vec<Step>,
}

/// Provides the default value for the `format` field
fn default_format() -> String {
    "json".to_string()
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
