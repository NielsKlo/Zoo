#[macro_use] extern crate serde_derive;

#[derive(Serialize, Deserialize, Debug)]
pub struct GameState {
    pub animal: Animal
}

impl GameState {
    pub fn tick_forward(&mut self) {
        let hunger = self.animal.hunger;
        self.animal.hunger = hunger - 1;
    }

    pub fn feed_animal(&mut self) {
        let hunger = self.animal.hunger;
        self.animal.hunger = hunger + 10;
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Animal {
    pub species: String,
    pub age: u32,
    pub hunger: u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_make_an_animal() {
        let animal = Animal{
            species: "tiger".to_string(),
            age: 5,
            hunger: 20
        };
        assert_eq!(animal.species, String::from("tiger"));
        assert_eq!(animal.age, 5);
        assert_eq!(animal.hunger, 20);
    }
}