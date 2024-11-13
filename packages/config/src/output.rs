use serde::Deserialize;

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

/// Provides the default value for the `format` field
fn default_format() -> String {
    "json".to_string()
}
