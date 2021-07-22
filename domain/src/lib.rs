#[macro_use] extern crate serde_derive;

#[derive(Serialize, Deserialize, Debug)]
pub struct GameState {
    pub animal: Animal
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Animal {
    pub species: String,
    pub age: i32,
    pub hunger: i32
}

impl GameState {
    pub fn tick_forward(&mut self) {
        let mut hunger = self.animal.hunger - 1;
        if hunger < 0 {
            hunger = 0;
        }
        self.animal.hunger = hunger;
    }

    pub fn feed_animal(&mut self) {
        let mut hunger = self.animal.hunger + 10;
        if hunger > 100 {
            hunger = 100;
        }
        self.animal.hunger = hunger;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_game_state(hunger: i32) -> GameState {
        GameState{
            animal: get_animal(hunger)
        }
    }

    fn get_animal(hunger: i32) -> Animal {
        Animal {
            species: "tiger".to_string(),
            age: 5,
            hunger
        }
    }

    #[test]
    fn can_make_an_animal() {
        let animal = get_animal(20);

        assert_eq!(animal.species, String::from("tiger"));
        assert_eq!(animal.age, 5);
        assert_eq!(animal.hunger, 20);
    }

    #[test]
    fn tick_forward_decreases_hunger_by_one() {
        let mut game_state = get_game_state(20);

        let old_hunger = game_state.animal.hunger;
        game_state.tick_forward();
        let new_hunger = game_state.animal.hunger;

        assert_eq!(new_hunger, old_hunger -1);
    }

    #[test]
    fn tick_forward_does_not_decrease_hunger_below_zero() {
        let mut game_state = get_game_state(0);

        let old_hunger = game_state.animal.hunger;
        game_state.tick_forward();
        let new_hunger = game_state.animal.hunger;

        assert_eq!(old_hunger, 0);
        assert_eq!(new_hunger, 0);
    }

    #[test]
    fn feed_animal_increases_hunger_by_ten() {
        let mut game_state = get_game_state(20);

        let old_hunger = game_state.animal.hunger;
        game_state.feed_animal();
        let new_hunger = game_state.animal.hunger;

        assert_eq!(new_hunger, old_hunger + 10);
    }

    #[test]
    fn feed_animal_does_not_increase_hunger_over_hundred() {
        let mut game_state = get_game_state(95);

        let old_hunger = game_state.animal.hunger;
        game_state.feed_animal();
        let new_hunger = game_state.animal.hunger;

        assert_eq!(old_hunger, 95);
        assert_eq!(new_hunger, 100);
    }
}
