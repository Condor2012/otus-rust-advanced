#[derive(Debug, PartialEq, Eq)]
pub enum List<A> {
    Nil,
    Cons(A, Box<List<A>>),
}

impl<A> List<A> {
    pub fn prepend(self, value: A) -> Self {
        Self::Cons(value, Box::new(self))
    }

    pub const fn head(&self) -> Option<&A> {
        match self {
            Self::Nil => None,
            Self::Cons(value, _) => Some(value),
        }
    }

    pub const fn len(&self) -> usize {
        match self {
            Self::Nil => 0,
            Self::Cons(_, tail) => 1 + tail.len(),
        }
    }

    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

pub struct Person {
    pub name: String,
    pub address: Option<String>,
}

pub fn name_and_address_len(person: Person) -> (String, usize) {
    match person {
        Person {
            name,
            address: Some(address),
        } => (name, address.len()),
        Person {
            name,
            address: None,
        } => (name, 0),
    }
}

pub fn age_type(age: Option<i32>) -> &'static str {
    match age {
        Some(value) if value < 0 => "Negative",
        Some(0..=18) => "Less or equal than 18",
        Some(value) if value > 150 => "Over 150",
        Some(_) => "Adult",
        None => "Empty",
    }
}

pub enum Color {
    Red,
    Yellow,
    Blue,
}

pub const fn say_color(color: &Color) -> &'static str {
    match color {
        Color::Red => "red",
        Color::Yellow => "yellow",
        Color::Blue => "blue",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recursive_list_uses_box() {
        let list = List::Nil.prepend(3).prepend(2).prepend(1);
        assert_eq!(list.head(), Some(&1));
        assert_eq!(list.len(), 3);
    }

    #[test]
    fn matching_handles_all_address_variants() {
        let person = Person {
            name: "Alice".into(),
            address: Some("Rust".into()),
        };
        assert_eq!(name_and_address_len(person), ("Alice".into(), 4));
    }

    #[test]
    fn age_matching_uses_guards_and_ranges() {
        assert_eq!(age_type(Some(-1)), "Negative");
        assert_eq!(age_type(Some(18)), "Less or equal than 18");
        assert_eq!(age_type(Some(42)), "Adult");
        assert_eq!(age_type(Some(151)), "Over 150");
        assert_eq!(age_type(None), "Empty");
    }

    #[test]
    fn color_matching_is_total() {
        assert_eq!(say_color(&Color::Blue), "blue");
    }
}
