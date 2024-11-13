use config::{Config, Output};
use serde_json::Value;

use crate::outputs;

pub fn output(config: Config, value: &Value) {
    for output in config.outputs {
        match output {
            Output::File { path, format } => {
                outputs::file_output::output_file(path, value.clone(), format)
            }
        }
    }
}
