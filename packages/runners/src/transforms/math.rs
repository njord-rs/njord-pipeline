use crate::state::RunState;

pub fn math(operation: String, run_state: &mut RunState) {
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

    run_state.data_state = serde_json::Value::Number(serde_json::Number::from_f64(result).unwrap());
}
