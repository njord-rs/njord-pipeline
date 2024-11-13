use crate::state::RunState;
use serde_json::Value;

/// Executes a JSON pointer query on the run state's data.
///
/// # Arguments
/// * `query` - A JSON pointer string to query the data.
/// * `run_state` - A mutable reference to the current run state.
pub fn json_query(query: String, run_state: &mut RunState) {
    run_state.data_state = run_state
        .data_state
        .pointer(&query)
        .cloned()
        .unwrap_or(Value::Null);
}
