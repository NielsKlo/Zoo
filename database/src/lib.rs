pub mod models;

use mongodb::sync::{Collection, Client};
use crate::models::game_state::GameState;

extern crate serde_json;

fn get_collection() -> Collection<GameState> {
    let client = Client::with_uri_str(
        "mongodb://localhost:27017",
    ).expect("Couldn't find mongodb at localhost:27017");

    let database = client.database("ZooDB");

    database.collection_with_type::<GameState>("Animals")
}

pub fn get_animal() -> String {
    let collection = get_collection();
    let cursor = collection.find_one(None, None).expect("Couldn't search for entries in the collection.");
    let game_state = cursor.expect("Collection did not have any entries.");
    serde_json::to_string(&game_state).unwrap()
}

pub fn save_animal(game_state: GameState) {
    let collection = get_collection();
    collection.insert_one(game_state, None).expect("Couldn't insert game state into database.");
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
}