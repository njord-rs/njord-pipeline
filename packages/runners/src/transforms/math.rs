use crate::state::RunState;

/// Executes a math operation on the run state's data.
///
/// # Arguments
///
/// * `operation` - The math operation to perform. Must be one of "sum" or "average".
/// * `decimal_places` - The number of decimal places to round to.
/// * `run_state` - A mutable reference to the current run state.   
///
/// # Returns
/// * `()`
///
/// # Examples
///
/// ```
/// tasks:
///   - source: weather
///     name: avg_temp
///     steps:
///       - type: json_query
///         query: /HttpResult/hourly/temperature_2m
///       - type: math
///         operation: average
/// ```
pub fn process(operation: &String, decimal_places: &Option<usize>, run_state: &mut RunState) {
    if run_state.data_state.is_null() {
        return;
    }

    if !run_state.data_state.is_array() {
        return;
    }

    let result = match operation.as_str() {
        "sum" => run_state
            .data_state
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_f64().unwrap())
            .sum::<f64>(),
        "average" => {
            run_state
                .data_state
                .as_array()
                .unwrap()
                .iter()
                .map(|v| v.as_f64().unwrap())
                .sum::<f64>()
                / run_state.data_state.as_array().unwrap().len() as f64
        }
        _ => {
            println!("Unknown math operation: {}", operation);
            0.0
        }
    };

    // Apply rounding if `decimal_places` is specified
    let rounded_result = if let Some(places) = decimal_places {
        let factor = 10_f64.powi(*places as i32);
        (result * factor).round() / factor
    } else {
        result
    };

    run_state.data_state =
        serde_json::Value::Number(serde_json::Number::from_f64(rounded_result).unwrap());
}
