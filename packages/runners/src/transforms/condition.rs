use config::step::Step;
use evalexpr::eval_boolean;

use crate::state::RunState;

use super::process_steps;

/// Processes a condition
///
/// # Arguments
///
/// * `condition` - The condition to evaluate
/// * `true_steps` - The steps to execute if the condition is true
/// * `false_steps` - The steps to execute if the condition is false
/// * `run_state` - The run state
///
/// # Returns
/// * `()`
pub fn process(
    condition: &str,
    true_steps: &Vec<Step>,
    false_steps: &Vec<Step>,
    run_state: &mut RunState,
) {
    if parse_condition(condition, run_state) {
        process_steps(&true_steps, run_state);
    } else {
        process_steps(&false_steps, run_state);
    }
}

/// Parses a condition
///
/// # Arguments
///
/// * `condition` - The condition to parse
///
/// # Returns
/// * `bool`
///
/// # Example
///
/// ```
/// let condition = "{{average_temp}} >= {{item.min_temperature}} && {{average_temp}} <= {{item.max_temperature}}";
/// let result = parse_condition(condition);
/// assert_eq!(result, true);
/// ```
///
/// {{variable_name}} is replaced with the value of the variable
/// {{variable_name.property}} is replaced with the value of the property of the variable called variable_name
fn parse_condition(condition: &str, run_state: &mut RunState) -> bool {
    // Helper function to resolve variable values from the run_state
    fn resolve_variable(key: &str, run_state: &RunState) -> Option<String> {
        if let Some((var, prop)) = key.split_once('.') {
            // Handle `{{variable.property}}` case
            if let Some(value) = run_state.variables.get(var) {
                if let Some(obj) = value.as_object() {
                    return obj.get(prop).and_then(|v| Some(v.to_string()));
                }
            }
        } else {
            // Handle `{{variable}}` case
            return run_state
                .variables
                .get(key)
                .and_then(|v| Some(v.to_string()));
        }
        None
    }

    // Replace all placeholders in the condition with their values
    let mut resolved_condition = condition.to_string();
    let re = regex::Regex::new(r"\{\{([\w\.]+)\}\}").unwrap();
    for caps in re.captures_iter(condition) {
        if let Some(matched) = caps.get(0) {
            if let Some(var_name) = caps.get(1) {
                let key = var_name.as_str();
                if let Some(value) = resolve_variable(key, run_state) {
                    resolved_condition = resolved_condition.replace(matched.as_str(), &value);
                } else {
                    panic!("Variable {} not found in run_state", key);
                }
            }
        }
    }

    match eval_boolean(&resolved_condition) {
        Ok(result) => result,
        Err(err) => {
            eprintln!("Failed to evaluate condition: {}", err);
            return false;
        }
    }
}
