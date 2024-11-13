use crate::state::RunState;
use csv::{ReaderBuilder, StringRecord};
use serde_json::{json, Value};

/// Converts data from one format to another.
///
/// # Arguments
/// * `from` - The current data format.
/// * `run_state` - A mutable reference to the current run state.
pub fn convert(from: String, run_state: &mut RunState) {
    let data = match from.as_str() {
        "csv" => read_as_csv(run_state),
        _ => {
            eprintln!("Unsupported format: {}", from);
            return;
        }
    };

    run_state.data_state = data;
}

/// Reads CSV data from the run state and converts it into JSON format.
fn read_as_csv(run_state: &mut RunState) -> Value {
    if run_state.data_state.is_null() {
        return Value::Null;
    }

    let raw_csv = run_state.data_state.to_string();

    println!("Raw CSV: {}", raw_csv);

    let mut reader = ReaderBuilder::new().from_reader(raw_csv.as_bytes());
    let mut rows = Vec::new();

    for result in reader.records() {
        match result {
            Ok(record) => rows.push(record_to_json(&record)),
            Err(err) => {
                eprintln!("Failed to read CSV record: {}", err);
                return json!({"error": "Invalid CSV data"});
            }
        }
    }

    json!(rows)
}

/// Converts a single CSV record into a JSON object.
fn record_to_json(record: &StringRecord) -> Value {
    let fields: Vec<String> = record.iter().map(String::from).collect();
    json!(fields)
}
