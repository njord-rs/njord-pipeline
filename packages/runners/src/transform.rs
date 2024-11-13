use std::collections::HashMap;

use crate::{state::RunState, transforms};
use config::{Config, Step};

pub fn transform(config: Config, run_state: &mut RunState) {
    let mut task_data_state = HashMap::new();

    for task in config.tasks {
        if let Some(json_value) = run_state.source_state.get(&task.source) {
            run_state.data_state =
                serde_json::to_value(json_value).expect("Failed to serialize source value");
        } else {
            println!("Source '{}' not found", task.source);
            continue;
        }

        for step in task.steps {
            match step {
                Step::JsonQuery { query } => transforms::json_query::json_query(query, run_state),
                Step::Math { operation } => transforms::math::math(operation, run_state),
                Step::Convert { from } => transforms::convert::convert(from, run_state),
            }
        }

        task_data_state.insert(task.name, run_state.data_state.clone());
    }
}
