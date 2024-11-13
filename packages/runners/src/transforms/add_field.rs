use crate::state::RunState;

/// Adds a field to the run state's data
///
/// # Arguments
///
/// * `field` - The field name
/// * `value` - The value of the field
/// * `run_state` - The run state
///
/// # Returns
/// * `()`
pub fn process(field: &String, value: serde_json::Value, run_state: &mut RunState) {
    if !run_state.data_state.is_object() {
        return;
    }

    run_state.data_state[field] = value;
}
