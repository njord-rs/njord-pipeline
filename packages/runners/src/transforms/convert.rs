use crate::state::RunState;
use csv::{ReaderBuilder, StringRecord};
use serde_json::{json, Value};

/// Converts data from one format to another.
///
/// # Arguments
/// * `from` - The current data format.
/// * `run_state` - A mutable reference to the current run state.
pub fn process(from: &String, run_state: &mut RunState) {
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

    let raw_csv = run_state.data_state.to_string().replace("\\n", "\n");
    // Remove first and last character
    let raw_csv = raw_csv[1..raw_csv.len() - 1].to_string();

    let mut reader = ReaderBuilder::new().from_reader(raw_csv.as_bytes());

    let headers = match reader.headers() {
        Ok(headers) => headers.clone(),
        Err(err) => {
            eprintln!("Failed to read headers: {}", err);
            return Value::Null;
        }
    };

    let records = match reader
        .records()
        .collect::<Result<Vec<StringRecord>, csv::Error>>()
    {
        Ok(records) => records,
        Err(err) => {
            eprintln!("Failed to parse CSV records: {}", err);
            return Value::Null;
        }
    };

    let mut rows = Vec::new();

    for record in records {
        rows.push(record_to_json(&headers, &record));
    }

    json!(rows)
}

/// Converts a single CSV record into a JSON object with headers as keys.
fn record_to_json(headers: &StringRecord, record: &StringRecord) -> Value {
    let mut obj = serde_json::Map::new();

    for (header, field) in headers.iter().zip(record.iter()) {
        // If field is a number, convert it to a Value::Number, if bool, Value::Bool, else, Value::String
        let json_field = match field.parse::<f64>() {
            Ok(num) => json!(num),
            Err(_) => match field.parse::<bool>() {
                Ok(bool) => Value::Bool(bool),
                Err(_) => Value::String(field.to_string()),
            },
        };

        obj.insert(header.to_string(), json_field);
    }

    Value::Object(obj)
}
