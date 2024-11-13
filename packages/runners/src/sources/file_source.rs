/// Get the contents of a file
///
/// # Arguments
/// * `path` - The path to the file
///
/// # Returns
/// * `String` - The contents of the file
pub fn process_file_source(path: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path)
}
