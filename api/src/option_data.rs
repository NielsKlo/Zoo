use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct OptionData {
    pub player: String,
    pub difficulty: i32
}