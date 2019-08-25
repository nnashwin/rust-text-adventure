use std::collections::HashMap;

#[derive(Debug)]
enum ItemState {
    Room,
    Inventory,
    Equipped,
}

#[derive(Debug)]
pub struct Item {
    name: String,
    description: String,
    weight: usize,
    location: ItemState,
}

impl Item {
    pub fn get_description(&self) -> &str {
        &self.description
    }

    pub fn equip(&mut self) {
        self.location = match self.location {
            ItemState::Room => ItemState::Room,
            _ => ItemState::Equipped,
        }
    }

    pub fn to_inventory(&mut self) {
        self.location = match self.location {
            _ => ItemState::Inventory,
        }
    }

    pub fn unequip(&mut self) {
        self.location = match self.location {
            ItemState::Room => ItemState::Room,
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
            weight: 30,
            location: ItemState::Room,
        },
    );

    map.insert(
        "buster",
        Item {
            name: "buster".to_string(),
            description: "A large cannon with four buttons".to_string(),
            weight: 20,
            location: ItemState::Room,
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
            weight: 30,
            location: ItemState::Room,
        };

        assert_eq!(new_item.get_description(), expected);
    }
}
