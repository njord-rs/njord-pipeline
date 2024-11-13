pub mod add_field;
pub mod condition;
pub mod convert;
pub mod json_query;
pub mod r#loop;
pub mod math;

use crate::{state::RunState, transforms};
use config::{step::Step, Config};
use std::collections::HashMap;

/// Runs the data transformation steps
pub fn transform(config: Config, run_state: &mut RunState) {
    let mut task_data_state = HashMap::new();

    set_variables(&config, run_state);

    for task in config.tasks {
        let task_name = task.name.clone().unwrap_or_default();
        print!("Processing task: {}", task_name);
        if !set_source_data(&task.source, run_state) {
            continue;
        }

        process_steps(&task.steps, run_state);

        if let Some(output) = task.output {
            run_state
                .variables
                .insert(output, run_state.data_state.clone());
        }

        task_data_state.insert(task_name, run_state.data_state.clone());

        // Print a checkmark in green
        print!("\x1b[32m");
        print!("\t✔︎\n");
        print!("\x1b[0m");
    }
}

/// Processes the transformation steps
///
/// # Arguments
///
/// * `steps` - A vector of transformation steps
/// * `run_state` - The run state
///
/// # Returns
/// * `()`
pub fn process_steps(steps: &[Step], run_state: &mut RunState) {
    for step in steps {
        match step {
            Step::JsonQuery { query } => transforms::json_query::process(query, run_state),
            Step::Math {
                operation,
                decimals,
            } => transforms::math::process(operation, decimals, run_state),
            Step::Convert { from } => transforms::convert::process(from, run_state),
            Step::Loop { body } => r#loop::process(body, run_state),
            Step::Condition {
                condition,
                true_steps,
                false_steps,
            } => condition::process(condition, true_steps, false_steps, run_state),
            Step::AddField { field, value } => {
                add_field::process(field, serde_json::to_value(value).unwrap(), run_state)
            }
        }
    }
}

/// Sets the variables in the run state
///
/// # Arguments
///
/// * `config` - The configuration object   
/// * `run_state` - The run state
///
/// # Returns
/// * `()`
fn set_variables(config: &Config, run_state: &mut RunState) {
    for variable in &config.variables {
        let json_value: serde_json::Value = serde_json::to_value(variable.1).unwrap();
        run_state
            .variables
            .insert(variable.0.to_string(), json_value);
    }
}

/// Sets the source data in the run state
///
/// # Arguments
///
/// * `source` - The source name
/// * `run_state` - The run state
///
/// # Returns
/// * `bool`
fn set_source_data(source: &str, run_state: &mut RunState) -> bool {
    if let Some(json_value) = run_state.source_state.get(source) {
        run_state.data_state = json_value.get_value();
        true
    } else {
        eprintln!("Source '{}' not found", source);
        false
    }
}
