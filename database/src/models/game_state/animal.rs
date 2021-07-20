use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Animal {
    pub species: String,
    pub age: u32,
    pub hunger: u32
}