pub struct Words {
    items: Vec<String>,
}

impl Words {
    pub fn new(items: Vec<String>) -> Self {
        Self { items }
    }

    pub fn iter(&self) -> WordsIterator<'_> {
        WordsIterator {
            index: 0,
            items: &self.items,
        }
    }
}

pub struct WordsIterator<'a> {
    index: usize,
    items: &'a [String],
}

impl<'a> Iterator for WordsIterator<'a> {
    type Item = &'a String;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.items.get(self.index);
        self.index += usize::from(item.is_some());
        item
    }
}

impl IntoIterator for Words {
    type Item = String;
    type IntoIter = std::vec::IntoIter<String>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::Words;

    #[test]
    fn custom_iterator_borrows_items() {
        let words = Words::new(vec!["Rust".into(), "iterator".into()]);
        assert_eq!(words.iter().count(), 2);
        assert_eq!(words.iter().next().map(String::as_str), Some("Rust"));
    }

    #[test]
    fn into_iterator_owns_items() {
        let words = Words::new(vec!["one".into(), "two".into()]);
        assert_eq!(words.into_iter().collect::<Vec<_>>(), ["one", "two"]);
    }
}
