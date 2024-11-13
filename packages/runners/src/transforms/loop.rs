use std::vec;

use config::step::Step;
use serde_json::Value;

use crate::state::RunState;

use super::process_steps;

/// Loops through the data and processes the steps
///
/// # Arguments
///
/// * `body` - The steps to process
/// * `run_state` - The run state
///
/// # Returns
/// * `()`
pub fn process(body: &Vec<Step>, run_state: &mut RunState) {
    if !run_state.data_state.is_array() {
        return;
    }

    let mut results = vec![];

    for item in run_state.data_state.as_array().unwrap() {
        let mut new_run_state = run_state.clone();
        new_run_state.data_state = item.clone();
        new_run_state.variables = run_state.variables.clone();

        // Add the item to the variables
        new_run_state
            .variables
            .insert("item".to_owned(), item.clone());

        process_steps(body, &mut new_run_state);

        results.push(new_run_state.data_state);
    }

    run_state.data_state = Value::Array(results);
}
