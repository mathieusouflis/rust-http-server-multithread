pub fn parse_string_to_vec(string: String, splitter: &str) -> Vec<String> {
    let filtered_lines: Vec<String> = string
        .split(splitter)
        .filter(|val| !val.trim().is_empty())
        .map(|s| s.to_string())
        .collect();

    filtered_lines
}
