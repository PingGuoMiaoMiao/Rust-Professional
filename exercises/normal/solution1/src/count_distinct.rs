pub fn new_count_distinct(input_str: &str) -> usize {
    let mut seen = std::collections::HashSet::new();
    for item in input_str.split(',') {
        seen.insert(item.trim());
    }
    seen.len()
}
