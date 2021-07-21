use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Animal {
    pub species: String,
    pub age: i32,
    pub hunger: i32
}