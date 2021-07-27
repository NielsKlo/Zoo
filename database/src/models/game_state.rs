pub mod animal;
use serde::{Deserialize, Serialize};
use animal::Animal;

#[derive(Debug, Serialize, Deserialize)]
pub struct GameState {
    pub player: String,
    pub level: i32,
    pub progress: i32,
    pub animals: Vec<Animal>,
    pub dead_animals: Vec<Animal>
}