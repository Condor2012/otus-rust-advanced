use std::collections::{BTreeSet, HashMap, HashSet};

pub fn count_chars(string: &str) -> HashMap<char, usize> {
    let mut counts = HashMap::new();
    for character in string.chars() {
        *counts.entry(character).or_insert(0) += 1;
    }
    counts
}

pub fn unique_chars(string: &str) -> HashSet<char> {
    string.chars().collect()
}

pub fn ordered_unique_items<T: Ord>(items: Vec<T>) -> BTreeSet<T> {
    items.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::{count_chars, ordered_unique_items, unique_chars};

    #[test]
    fn entry_api_counts_characters() {
        let counts = count_chars("abacaba");
        assert_eq!(counts.get(&'a'), Some(&4));
        assert_eq!(counts.get(&'b'), Some(&2));
        assert_eq!(counts.get(&'c'), Some(&1));
    }

    #[test]
    fn from_iterator_builds_sets() {
        assert_eq!(unique_chars("abacaba").len(), 3);
        let ordered = ordered_unique_items(vec![3, 1, 3, 2, 1]);
        assert_eq!(ordered.into_iter().collect::<Vec<_>>(), [1, 2, 3]);
    }
}
