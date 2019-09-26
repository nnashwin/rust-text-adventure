use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub enum ItemState {
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

    pub fn get_location(&self) -> &ItemState {
        &self.location
    }

    pub fn get_name(&self) -> &str {
        &self.name
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

    #[test]
    fn test_get_name() {
        let expected = "test name".to_string();

        let new_item = Item {
            name: expected.clone(),
            description: "test desc".to_string(),
            weight: 30,
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
            weight: 30,
            location: ItemState::Room,
        };

        assert_eq!(new_item.get_location(), &expected);
    }

    #[test]
    fn test_equip() {
        let expected = ItemState::Equipped;

        let new_item = &mut Item {
            name: "test".to_string(),
            description: "test desc".to_string(),
            weight: 30,
            location: ItemState::Inventory,
        };

        new_item.equip();

        assert_eq!(new_item.get_location(), &expected);
    }

    fn test_equip_negative() {
        let expected = ItemState::Room;

        let new_item = &mut Item {
            name: "test".to_string(),
            description: "test desc".to_string(),
            weight: 30,
            location: ItemState::Room,
        };

        new_item.equip();

        assert_eq!(new_item.get_location(), &expected);
    }
}
