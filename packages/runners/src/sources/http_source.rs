use serde_json::Value;
use std::collections::HashMap;

/// Get the contents of a URL and process it based on the specified format.
///
/// # Arguments
/// * `url` - The URL to fetch contents from.
/// * `format` - The desired format for the content (e.g., "json").
///
/// # Returns
/// * `Result<String, Error>` - The processed content or an error.
pub async fn process_http_source(
    url: &str,
    format: &str,
) -> Result<HashMap<String, Value>, Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?.text().await?;

    match format {
        "json" => {
            let json = process_json(&response)?;
            Ok(json)
        },
        "html" => {
            let html = process_html(&response);
            Ok(HashMap::from([("html".to_string(), html)]))
        },
        _ => Ok(HashMap::new()),
    }
}

/// Process JSON data
///
/// # Arguments
/// * `json` - JSON data to process
///
/// # Returns
/// * `Result<HashMap<String, Value>, serde_json::Error>`
fn process_json(json: &str) -> Result<HashMap<String, Value>, serde_json::Error> {
    serde_json::from_str(json)
}

/// Process HTML data
///
/// # Arguments
/// * `html` - HTML data to process
///
/// # Returns
/// * `Value`
fn process_html(html: &str) -> Value {
    Value::String(html.to_string())
}
