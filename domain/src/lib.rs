#[macro_use] extern crate serde_derive;
use rand::Rng;
pub mod names;

#[derive(Serialize, Deserialize, Debug)]
pub struct GameState {
    pub player: String,
    pub score: i32,
    pub animals: Vec<Animal>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Animal {
    pub id: i32,
    pub species: String,
    pub name: String,
    pub hunger: i32
}

impl GameState {
    pub fn tick_forward(&mut self) {
        for i in 0..self.animals.len() {
            Self::progress_hunger(self, i);
            Self::progress_score(self, i);
        }

        if Self::deserves_new_animal(self){
            Self::generate_random_animal(self);
        }
    }

    fn progress_hunger(&mut self, i: usize){
        if self.animals[i].hunger > 0 {
            self.animals[i].hunger -= 1;
        }
    }

    fn progress_score(&mut self, i: usize){
        if self.animals[i].hunger >= 29 && self.animals[i].hunger <= 94 {
            self.score += 1;
        }
    }

    fn deserves_new_animal(&mut self) -> bool {
        let animal_count = self.animals.len() as i32;

        if animal_count >= 30 {
            return false;
        }

        let leftover_score = Self::get_leftover_score(self, animal_count);
        return leftover_score >= animal_count * 30
    }

    fn get_leftover_score(&self, animal_count: i32) -> i32 {
        let mut minimum_score = 0;
        for i in 0..(animal_count) {
            minimum_score += 30 * i;
        }
        self.score - minimum_score
    }

    fn generate_random_animal(&mut self){
        let array: [&str; 10] = ["penguin", "elephant", "bat", "crocodile", "deer", "dolphin", "giraffe", "monkey", "otter", "tiger"];
        let mut rng = rand::thread_rng();
        let random_number = rng.gen_range(0..array.len());
        let animal = Animal {
            id: self.animals.len() as i32,
            species: array[random_number].to_string(),
            name: names::get_random_name(),
            hunger: 50
        };
        self.animals.push(animal);
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
            score: 40,
            animals: get_animal(hunger)
        }
    }

    fn get_animal(hunger: i32) -> Vec<Animal> {
        vec! [Animal {
            id: 0,
            species: "tiger".to_string(),
            name: "Jacky".to_string(),
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
    fn tick_forward_adds_a_new_animal_when_score_is_high_enough() {
        let mut game_state = get_game_state(0);

        let starting_animal_count = game_state.animals.len();
        game_state.tick_forward();
        let intermediate_animal_count = game_state.animals.len();
        game_state.tick_forward();
        let ending_animal_count = game_state.animals.len();

        assert_eq!(intermediate_animal_count, starting_animal_count + 1);
        assert_eq!(ending_animal_count, intermediate_animal_count);
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
