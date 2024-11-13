use serde::Deserialize;

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

/// Provides the default value for the `format` field
fn default_format() -> String {
    "json".to_string()
}
