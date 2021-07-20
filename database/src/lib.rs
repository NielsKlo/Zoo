mod models;

use mongodb::sync::{Collection, Client};
use crate::models::game_state::GameState;
extern crate serde_json;

fn get_connection() -> Collection<GameState> {
    let client = Client::with_uri_str(
        "mongodb://localhost:27017",
    ).expect("Couldn't find mongodb at localhost:27017");

    let database = client.database("ZooDB");

    database.collection_with_type::<GameState>("Animals")
}

pub fn get_animal() -> String {
    let collection = get_connection();
    let cursor = collection.find_one(None, None).expect("Couldn't search for entries in the collection.");
    let game_state = cursor.expect("Collection did not have any entries.");
    println!("{}", game_state.animal.species);
    serde_json::to_string(&game_state).unwrap()
}