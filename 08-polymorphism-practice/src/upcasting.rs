//! Нативный trait upcasting, доступный начиная с Rust 1.86.

pub trait Animal {
    fn name(&self) -> &str;
}

pub trait Pet: Animal {
    fn owner(&self) -> &str;
}

pub struct Dog {
    pub name: String,
    pub owner: String,
}

impl Animal for Dog {
    fn name(&self) -> &str {
        &self.name
    }
}

impl Pet for Dog {
    fn owner(&self) -> &str {
        &self.owner
    }
}

pub fn describe(pet: &dyn Pet) -> String {
    let animal = pet as &dyn Animal;
    format!("Животное: {}\nВладелец: {}", animal.name(), pet.owner())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn upcasts_pet_to_animal() {
        let dog = Dog {
            name: "Барсик".to_string(),
            owner: "Юрий".to_string(),
        };
        assert_eq!(describe(&dog), "Животное: Барсик\nВладелец: Юрий");
    }
}
