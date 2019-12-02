use serde_derive::{Serialize, Deserialize};
use super::examine::{Examine};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Interactable {
    pub after_interaction_description: String,
    pub before_interaction_description: String,
    pub id: String,
    pub interaction_description: String,
    pub interacted: bool,
    pub name: String,
    pub prerequisite_item: String,
}

impl Interactable {
    pub fn interact(&mut self) {
        self.interacted = true
    }

    pub fn is_interacted(&self) -> bool {
        self.interacted
    }
}

impl Examine for Interactable {
    fn examine(&self) -> String {
        if self.interacted {
            self.after_interaction_description.clone()
        } else {
            self.before_interaction_description.clone()
        }
    }
}
