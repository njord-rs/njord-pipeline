use serde::Deserialize;

use crate::step::Step;

/// The different tasks
#[derive(Debug, Deserialize, Clone)]
pub struct Task {
    pub name: Option<String>,
    pub source: String,
    pub steps: Vec<Step>,
    pub output: Option<String>,
}
