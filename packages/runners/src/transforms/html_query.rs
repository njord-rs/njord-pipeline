use crate::state::RunState;
use scraper::{Html, Selector};
use serde_json::Value;

/// Executes an HTML query on the run state's data.
///
/// # Arguments
/// * `selector` - An HTML selector to query the data.
/// * `run_state` - A mutable reference to the current run state.
///
/// # Returns
/// * `()`
pub fn process(selector: &String, run_state: &mut RunState) {
    let document = Html::parse_document(&run_state.data_state.to_string());
    let parsed_selector = Selector::parse(selector).unwrap();

    // Store the HTML selected elements as HTML
    let elements = document.select(&parsed_selector);

    let mut inner_html = String::new();

    inner_html.push_str("<div>");

    for element in elements {
        inner_html.push_str(element.inner_html().as_str());
    }

    inner_html.push_str("</div>");

    run_state.data_state = Value::String(inner_html);
}
