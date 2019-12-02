use serde_derive::{Serialize, Deserialize};
use super::exit::*;
use super::interactable::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Room {
    pub description: String,
    pub interactables: Vec<Interactable>,
    pub items: Vec<String>,
    pub exits: Vec<Exit>,
}

impl Room {
    pub fn get_description(&self) -> &str {
        &self.description
    }
}
