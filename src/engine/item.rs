use std::collections::HashMap;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ItemState {
    Room,
    Inventory,
}

#[derive(Clone, Debug)]
pub struct Item {
    pub name: String,
    pub description: String,
    pub location: ItemState,
}

impl Item {
    pub fn get_description(&self) -> &str {
        &self.description
    }

    pub fn get_location(&self) -> &ItemState {
        &self.location
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn is_in_inventory(&self) -> bool {
        self.location == ItemState::Inventory
    }

    pub fn to_inventory(&mut self) {
        self.location = match self.location {
            _ => ItemState::Inventory,
        }
    }
}

pub fn create_inventory() -> HashMap<&'static str, Item> {
    let mut map = HashMap::new();

    map.insert(
        "helmet",
        Item {
            name: "helmet".to_string(),
            description: "a blue helmet covered in dirt".to_string(),
            location: ItemState::Room,
        },
    );

    map.insert(
        "buster",
        Item {
            name: "buster".to_string(),
            description: "A large cannon with four buttons".to_string(),
            location: ItemState::Room,
        },
    );

    map.insert(
        "pendant",
        Item {
            name: "pendant".to_string(),
            description: "A rusty pendant with a small seal on it.".to_string(),
            location: ItemState::Inventory,
        },
    );

    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_description() {
        let expected = "test desc".to_string();

        let new_item = Item {
            name: "test".to_string(),
            description: expected.clone(),
            location: ItemState::Room,
        };

        assert_eq!(new_item.get_description(), expected);
    }

    #[test]
    fn test_get_name() {
        let expected = "test name".to_string();

        let new_item = Item {
            name: expected.clone(),
            description: "test desc".to_string(),
            location: ItemState::Room,
        };

        assert_eq!(new_item.get_name(), expected);
    }

    #[test]
    fn test_get_location() {
        let expected = ItemState::Room;

        let new_item = Item {
            name: "test".to_string(),
            description: "test desc".to_string(),
            location: ItemState::Room,
        };

        assert_eq!(new_item.get_location(), &expected);
    }
}
