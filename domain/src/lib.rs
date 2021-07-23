#[macro_use] extern crate serde_derive;

#[derive(Serialize, Deserialize, Debug)]
pub struct GameState {
    pub player: String,
    pub animals: Vec<Animal>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Animal {
    pub id: i32,
    pub species: String,
    pub hunger: i32
}

impl GameState {
    pub fn tick_forward(&mut self) {
        for i in 0..self.animals.len() {
            if self.animals[i].hunger > 0 {
                self.animals[i].hunger = self.animals[i].hunger - 1;
            }
        }
    }

    pub fn feed_animal(&mut self, id: usize) {
        let mut hunger = self.animals[id].hunger + 10;
        if hunger > 100 {
            hunger = 100;
        }
        self.animals[id].hunger = hunger;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_game_state(hunger: i32) -> GameState {
        GameState{
            player: "niels".to_string(),
            animals: get_animal(hunger)
        }
    }

    fn get_animal(hunger: i32) -> Vec<Animal> {
        vec! [Animal {
            id: 0,
            species: "tiger".to_string(),
            hunger
        }]
    }

    #[test]
    fn can_make_an_animal() {
        let animals = get_animal(20);
        let animal = &animals[0];

        assert_eq!(animal.id, 0);
        assert_eq!(animal.species, String::from("tiger"));
        assert_eq!(animal.hunger, 20);
    }

    #[test]
    fn tick_forward_decreases_hunger_by_one() {
        let mut game_state = get_game_state(20);

        let old_hunger = game_state.animals[0].hunger;
        game_state.tick_forward();
        let new_hunger = game_state.animals[0].hunger;

        assert_eq!(new_hunger, old_hunger -1);
    }

    #[test]
    fn tick_forward_does_not_decrease_hunger_below_zero() {
        let mut game_state = get_game_state(0);

        let old_hunger = game_state.animals[0].hunger;
        game_state.tick_forward();
        let new_hunger = game_state.animals[0].hunger;

        assert_eq!(old_hunger, 0);
        assert_eq!(new_hunger, 0);
    }

    #[test]
    fn feed_animal_increases_hunger_by_ten() {
        let mut game_state = get_game_state(20);

        let old_hunger = game_state.animals[0].hunger;
        game_state.feed_animal(0);
        let new_hunger = game_state.animals[0].hunger;

        assert_eq!(new_hunger, old_hunger + 10);
    }

    #[test]
    fn feed_animal_does_not_increase_hunger_over_hundred() {
        let mut game_state = get_game_state(95);

        let old_hunger = game_state.animals[0].hunger;
        game_state.feed_animal(0);
        let new_hunger = game_state.animals[0].hunger;

        assert_eq!(old_hunger, 95);
        assert_eq!(new_hunger, 100);
    }
}
