pub mod models;

use mongodb::sync::{Collection, Client};
use mongodb::bson::doc;
use crate::models::game_state::GameState;

extern crate serde_json;

fn get_collection() -> Collection<GameState> {
    let client = Client::with_uri_str(
        "mongodb://localhost:27017",
    ).expect("Couldn't find mongodb at localhost:27017");

    let database = client.database("ZooDB");

    database.collection_with_type::<GameState>("Animals")
}

pub fn get_animals(name: String) -> String {
    let collection = get_collection();
    let option = collection.find_one(doc! {"player": &name}, None)
        .expect("Couldn't search for entries in the collection.");
    let game_state;
    match option {
        Some(x) => game_state = x,
        None => game_state = get_starting_state(collection, name)
    };
    serde_json::to_string(&game_state).unwrap()
}

fn get_starting_state(collection: Collection<GameState>, name: String) -> GameState {
    let option = collection.find_one(doc! {"player": "default, dont save"}, None)
        .expect("Couldn't search for entries in the collection.");
    let mut game_state = option.expect("Couldn't find the default game state");
    game_state.player = name;
    game_state
}

pub fn save_animal(game_state: GameState) {
    let collection = get_collection();
    let name = &game_state.player;
    collection.delete_one(doc! {"player": name} , None).expect("Couldn't delete the game state from the database");
    collection.insert_one(game_state, None)
        .expect("Couldn't insert the new game state into database.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_collection_returns_the_correct_collection() {
        let collection = get_collection();
        let name = collection.name();
        assert_eq!(name, "Animals");
    }

    #[test]
    fn get_starting_state_returns_the_default_starting_document(){
        let collection = get_collection();
        let game_state = get_starting_state(collection, "test name".to_string());
        let default_first_species = &game_state.animals[0].species;
        assert_eq!(default_first_species, "penguin");
    }
}