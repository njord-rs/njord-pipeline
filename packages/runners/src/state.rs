use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::sources::SourceState;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RunState {
    pub source_state: HashMap<String, SourceState>,
    pub data_state: Value,
    pub variables: HashMap<String, Value>,
}
