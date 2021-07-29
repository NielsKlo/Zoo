#[macro_use] extern crate serde_derive;
use rand::Rng;
pub mod names;

#[derive(Serialize, Deserialize, Debug)]
pub struct GameState {
    pub player: String,
    pub level: i32,
    pub progress: i32,
    pub difficulty: i32,
    pub animals: Vec<Animal>,
    pub dead_animals: Vec<Animal>
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
        let animals_died = Self::age_animals(self);

        if animals_died {
            Self::bury_dead_animals(self);
            if Self::no_animals_alive(self) {
                Self::generate_random_animal(self);
            }
        } else {
            Self::make_progress(self);
        }

        if Self::deserves_new_animal(self){
            Self::generate_random_animal(self);
        }
    }

    fn age_animals(&mut self) -> bool {
        for i in 0..self.animals.len() {
            Self::tick_forward_hunger(self, i);
            if self.animals[i].hunger == 0 {
                return true;
            }
        }
        return false;
    }

    fn tick_forward_hunger(&mut self, i: usize){
        if self.animals[i].hunger > 0 {
            self.animals[i].hunger -= 1;
        }
    }

    fn bury_dead_animals(&mut self){
        let mut dead_animals_present = true;
        while dead_animals_present {

            dead_animals_present = false;

            for i in 0..self.animals.len(){
                if self.animals[i].hunger == 0 {

                    let dead_animal = self.animals.remove(i);
                    self.dead_animals.push(dead_animal);
                    dead_animals_present = true;
                    break;
                }
            }
        }
        self.progress = 0;
    }

    fn no_animals_alive(&self) -> bool {
        let no_living_animals = self.animals.len() == 0;
        let below_animal_limit = (self.animals.len() + self.dead_animals.len()) < 26;
        no_living_animals && below_animal_limit
    }

    fn make_progress(&mut self) {
        let new_progress = Self::get_new_progress(self);
        let multiplier = Self::get_progress_multiplier(self);
        self.progress += ((new_progress as f32) * multiplier) as i32;
    }

    fn get_new_progress(&mut self) -> i32 {
        let mut new_progress = 0;
        for i in 0..self.animals.len() {
            new_progress += Self::get_animal_progress(self.animals[i].hunger);
        }
        (new_progress * 40) / (self.animals.len() as i32)
    }

    fn get_animal_progress(hunger: i32) -> i32 {
        if hunger >= 96 {
            5
        } else if hunger >= 31 {
            10
        } else if hunger >= 10 {
            3
        } else {
            1
        }
    }

    fn get_progress_multiplier(&self) -> f32 {
        let difference = (self.level - (self.animals.len() as i32)) as f32;
        let bonus;
        if difference > 5.0 {
            bonus = 1.0;
        } else {
            bonus = difference * 0.2;
        }
        1.0 + bonus
    }

    fn deserves_new_animal(&mut self) -> bool {
        let enough_progress = self.progress >= 10000;
        let below_animal_limit = (self.animals.len() + self.dead_animals.len()) < 26;
        enough_progress && below_animal_limit
    }

    fn generate_random_animal(&mut self) {
        let array: [&str; 10] = ["penguin", "elephant", "bat", "crocodile", "deer", "dolphin", "giraffe", "monkey", "otter", "tiger"];
        let mut rng = rand::thread_rng();
        let random_number = rng.gen_range(0..array.len());
        let animal = Animal {
            id: (self.animals.len() + self.dead_animals.len()) as i32,
            species: array[random_number].to_string(),
            name: names::get_random_name(),
            hunger: 50
        };
        self.animals.push(animal);
        Self::reset_progress(self);
    }

    fn reset_progress(&mut self) {
        self.progress = 0;
        if self.level < self.animals.len() as i32 {
            self.level = self.animals.len() as i32;
        }
    }

    pub fn feed_animal(&mut self, id: usize) {
        let mut hunger = self.animals[id].hunger + 10;
        if hunger > 100 {
            hunger = 100;
        }
        self.animals[id].hunger = hunger;
    }

    pub fn bulk_feed_animal(&mut self, id: usize) {
        let mut hunger = self.animals[id].hunger + 50;
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
            level: 1,
            progress: 1000,
            difficulty: 1,
            animals: get_animals(hunger),
            dead_animals: vec![]
        }
    }

    fn get_animals(hunger: i32) -> Vec<Animal> {
        vec! [Animal {
            id: 0,
            species: "tiger".to_string(),
            name: "Jacky".to_string(),
            hunger
        }]
    }

    #[test]
    fn can_make_an_animal() {
        let animals = get_animals(20);
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
    fn animals_die_when_they_reach_zero_hunger() {
        let mut game_state = get_game_state(1);

        let starting_dead_animal_count = game_state.dead_animals.len();
        game_state.tick_forward();
        let ending_dead_animal_count = game_state.dead_animals.len();

        assert_eq!(starting_dead_animal_count, 0);
        assert_eq!(ending_dead_animal_count, 1);
    }

    #[test]
    fn tick_forward_adds_a_new_animal_when_progress_is_high_enough() {
        let mut game_state = get_game_state(20);
        game_state.progress = 10000;

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

    #[test]
    fn bulk_feed_animal_increases_hunger_by_fifty() {
        let mut game_state = get_game_state(20);

        let old_hunger = game_state.animals[0].hunger;
        game_state.bulk_feed_animal(0);
        let new_hunger = game_state.animals[0].hunger;

        assert_eq!(new_hunger, old_hunger + 50);
    }

    #[test]
    fn bulk_feed_animal_does_not_increase_hunger_over_hundred() {
        let mut game_state = get_game_state(95);

        let old_hunger = game_state.animals[0].hunger;
        game_state.bulk_feed_animal(0);
        let new_hunger = game_state.animals[0].hunger;

        assert_eq!(old_hunger, 95);
        assert_eq!(new_hunger, 100);
    }

    #[test]
    fn a_new_animal_gets_generated_when_there_are_no_more_living_animals() {
        let mut game_state = get_game_state(1);

        let old_animal_name = game_state.animals[0].name.clone();
        game_state.tick_forward();
        let new_animal_name = game_state.animals[0].name.clone();

        assert_ne!(new_animal_name, old_animal_name);
    }

    #[test]
    fn progress_gets_set_to_zero_when_an_animal_dies() {
        let mut game_state = get_game_state(1);

        game_state.tick_forward();

        assert_eq!(game_state.progress, 0);
    }
}
