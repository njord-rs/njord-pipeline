use std::collections::HashMap;

use config::Config;
use serde_json::Value;

use crate::{outputs, sources, state::RunState, transforms};

pub async fn run(config: Config) {
    let source_state = sources::process_sources(config.clone()).await;
    let mut run_state = RunState {
        source_state,
        data_state: Value::Null,
        variables: HashMap::new(),
    };

    transforms::transform(config.clone(), &mut run_state);

    outputs::output(config, &run_state.data_state);
}
