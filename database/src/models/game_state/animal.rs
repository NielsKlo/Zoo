use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Animal {
    pub id: i32,
    pub species: String,
    pub hunger: i32
}