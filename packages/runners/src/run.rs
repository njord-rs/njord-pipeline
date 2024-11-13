use config::Config;
use serde_json::Value;

use crate::{output, source, state::RunState, transform};

pub async fn run(config: Config) {
    let source_state = source::process_sources(config.clone()).await;
    let mut run_state = RunState {
        source_state,
        data_state: Value::Null,
    };

    transform::transform(config.clone(), &mut run_state);

    output::output(config, &run_state.data_state);
}
