#[macro_use] extern crate serde_derive;

#[derive(Serialize, Deserialize)]
pub struct GameState {
    pub animal: Animal
}

#[derive(Serialize, Deserialize)]
pub struct Animal {
    pub species: String,
    pub age: u32,
    pub hunger: u32
}