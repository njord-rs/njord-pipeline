use serde::Deserialize;
use serde_yaml::Value;

/// The different steps in the data pipeline
#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum Step {
    #[serde(rename = "json_query")]
    JsonQuery { query: String },
    #[serde(rename = "math")]
    Math {
        operation: String,
        decimals: Option<usize>,
    },
    #[serde(rename = "convert")]
    Convert { from: String },
    #[serde(rename = "loop")]
    Loop { body: Vec<Step> },
    #[serde(rename = "condition")]
    Condition {
        condition: String,
        #[serde(rename = "true_steps")]
        true_steps: Vec<Step>,
        #[serde(rename = "false_steps")]
        false_steps: Vec<Step>,
    },
    #[serde(rename = "add_field")]
    AddField { field: String, value: Value },
}
