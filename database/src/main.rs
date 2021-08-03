use mongodb::sync::{Collection, Client};
use mongodb::bson::doc;
use database::models::game_state::GameState;
use database::models::game_state::animal::Animal;

fn main () {
    let collection = get_collection();
    let game_state = get_game_state();
    collection.delete_one(doc! {"player": "default, dont save"}, None).expect("");
    collection.insert_one(game_state, None)
        .expect("Couldn't insert the new game state into database.");
}

fn get_collection() -> Collection<GameState> {
    let client = Client::with_uri_str("mongodb://localhost:27017")
        .expect("Couldn't find mongodb at localhost:27017");

    let database = client.database("ZooDB");

    database.collection_with_type::<GameState>("Animals")
}

fn get_game_state() -> GameState {
    GameState {
        player: "default, dont save".to_string(),
        level: 1,
        progress: 0,
        difficulty: 1,
        animals: vec! [Animal {
            id: 0,
            species: "penguin".to_string(),
            name: "Henry".to_string(),
            hunger: 50
        }],
        dead_animals: vec![]
    }
}