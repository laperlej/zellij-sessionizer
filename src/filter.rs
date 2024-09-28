pub fn fuzzy_filter(items: &[String], search_term: &str) -> Vec<String> {
    items.iter()
        .filter(|item| item.contains(search_term))
        .map(|item| item.to_string())
        .collect()
}
