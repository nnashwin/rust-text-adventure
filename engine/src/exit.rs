use serde_derive::{Serialize, Deserialize};
use super::direction::Direction;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Exit {
    pub direction: Direction,
    pub locked: bool,
    pub interactable_id: String,
    pub target: usize,
}

impl Exit {
    pub fn is_locked(&self) -> bool {
        self.locked
    }

    pub fn unlock(&mut self) {
        self.locked = false
    }
}
