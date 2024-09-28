use nucleo_matcher::{Config, Matcher};
use nucleo_matcher::pattern::{CaseMatching, Normalization, Pattern};


//from https://docs.rs/nucleo-matcher/0.3.1/nucleo_matcher/
//
// let paths = ["foo/bar", "bar/foo", "foobar"];
// let mut matcher = Matcher::new(Config::DEFAULT.match_paths());
// let matches = Pattern::parse("foo bar", CaseMatching::Ignore, Normalization::Smart).match_list(paths, &mut matcher);
// assert_eq!(matches, vec![("foo/bar", 168), ("bar/foo", 168), ("foobar", 140)]);
// let matches = Pattern::parse("^foo bar", CaseMatching::Ignore, Normalization::Smart).match_list(paths, &mut matcher);
// assert_eq!(matches, vec![("foo/bar", 168), ("foobar", 140)]);

pub fn fuzzy_filter(items: &[String], search_term: &str) -> Vec<String> {
    if search_term.is_empty() {
        let sorted = items.iter().map(|item| item.to_string()).collect::<Vec<_>>();
        return sorted;
    }
    let mut matcher = Matcher::new(Config::DEFAULT.match_paths());
    let mut matches = Pattern::parse(search_term, CaseMatching::Ignore, Normalization::Smart).match_list(items, &mut matcher);
    matches.sort_by(|a, b| a.1.cmp(&b.1));
    matches.into_iter().map(|(item, _)| item.to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuzzy_filter() {
        let items: Vec<String> = vec!["/home/laperlej/Projects/bioblend", "/home/laperlej/Projects/backup-rotation", "/home/laperlej/Projects/github.io"].into_iter().map(|item| item.to_string()).collect();
        let search_term = "bio";
        let result = fuzzy_filter(&items, search_term);
        assert_eq!(result, vec!["/home/laperlej/Projects/bioblend"]);
    }
}
