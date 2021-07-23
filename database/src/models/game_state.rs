pub mod animal;
use serde::{Deserialize, Serialize};
use animal::Animal;

#[derive(Debug, Serialize, Deserialize)]
pub struct GameState {
    pub player: String,
    pub animals: Vec<Animal>
}