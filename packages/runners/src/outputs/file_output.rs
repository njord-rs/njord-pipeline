use serde_json::Value;

/// Outputs results to file
///
/// # Arguments
///
/// * `path` - Path to file
/// * `value` - Value
/// * `format` - Format
pub fn output_file(path: String, value: Value, format: String) {
    let results = match format.as_str() {
        "json" => serde_json::to_string_pretty(&value).unwrap(),
        "yaml" => serde_yaml::to_string(&value).unwrap(),
        _ => panic!("Unknown format {}", format),
    };
    std::fs::write(path, results).unwrap();
}
