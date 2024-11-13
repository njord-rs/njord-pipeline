use std::collections::HashMap;

use config::{Config, Source};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::sources;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SourceState {
    FileResult(Value),
    HttpResult(HashMap<String, Value>),
}

impl SourceState {
    pub fn get_value(&self) -> &Value {
        match self {
            SourceState::FileResult(value) => value,
            SourceState::HttpResult(value) => value,
        }
    }
}

pub async fn process_sources(config: Config) -> HashMap<String, SourceState> {
    let mut results = HashMap::new();

    for source in config.sources {
        let result = match source {
            Source::File { path, name } => {
                (SourceState::FileResult(process_file_source(path)), name)
            }
            Source::Http { url, format, name } => (
                SourceState::HttpResult(process_http_source(url, format).await),
                name,
            ),
        };

        results.insert(result.1, result.0);
    }

    results
}

async fn process_http_source(url: String, format: String) -> HashMap<String, Value> {
    match sources::http_source::process_http_source(url.as_str(), format.as_str()).await {
        Ok(results) => results,
        Err(err) => panic!("{}", err),
    }
}

fn process_file_source(path: String) -> Value {
    match sources::file_source::process_file_source(path.as_str()) {
        Ok(results) => Value::String(results),
        Err(err) => panic!("{}", err),
    }
}
