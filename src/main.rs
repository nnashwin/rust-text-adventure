use std::collections::HashMap;
use std::io;

mod commands;

#[derive(PartialEq)]
enum Direction {
    N,
    S,
    E,
    W,
    NE,
    NW,
    SE,
    SW,
}

fn is_legal_command(command: &str) -> bool {
    return commands::legal_commands::LEGAL_COMMANDS.contains_key(command);
}

fn is_obj_noun<'a>(word: &'a str, item_map: &HashMap<String, Item>) -> bool {
    return item_map.contains_key(word);
}

struct Exit {
    direction: Direction,
    target: i32,
    locked: bool,
    key: String,
}

impl Exit {
    fn can_go(&self, direction: &Direction) -> bool {
        self.direction == *direction && !self.locked
    }
}

#[derive(Debug, Default)]
struct Input {
    intent: String,
    object_noun: String,
}

#[derive(Debug)]
struct Interactable {
    name: String,
    before_interaction_description: String,
    after_interaction_description: String,
    interacted: bool,
}

impl Interactable {
    fn get_description(&self) -> &String {
        match self.interacted {
            true => &self.before_interaction_description,
            false => &self.after_interaction_description,
        }
    }
}

#[derive(Debug)]
enum ItemState {
    Room,
    Inventory,
    Equipped,
}

#[derive(Debug)]
struct Item {
    name: String,
    description: String,
    weight: i32,
    location: ItemState,
}

fn get_item_vec() -> Vec<(String, Item)> {
    return vec![
        (
            "helmet".to_string(),
            Item {
                name: "helmet".to_string(),
                description: "A blue helmet covered in dirt".to_string(),
                weight: 30,
                location: ItemState::Room,
            },
        ),
        (
            "buster".to_string(),
            Item {
                name: "buster".to_string(),
                description: "A large cannon with four buttons".to_string(),
                weight: 20,
                location: ItemState::Room,
            },
        ),
    ];
}

impl Item {
    fn to_inventory(&mut self) {
        self.location = match self.location {
            _ => ItemState::Inventory,
        }
    }

    fn equip(&mut self) {
        self.location = match self.location {
            ItemState::Room => ItemState::Room,
            _ => ItemState::Equipped,
        }
    }

    fn unequip(&mut self) {
        self.location = match self.location {
            ItemState::Room => ItemState::Room,
            _ => ItemState::Inventory,
        }
    }
}

struct Room {
    description: String,
    interactables: Vec<Interactable>,
    items: Vec<Item>,
    exits: Vec<Exit>,
}

fn main() {
    let mut rooms = vec![
            Room {
                description: "You find yourself in a room. There is a door to the south and a door to the east.".to_string(),
                exits: vec![
                    Exit {
                        direction: Direction::S,
                        target: 2, locked: false,
                        key: String::from(""),
                    },
                    Exit {
                        direction: Direction::E,
                        target: 1,
                        locked: false,
                        key: String::from(""),
                    },
                ],
                interactables: vec![],
                items: vec![],
            },
            Room {
                description: "You find yourself in a room. There is a door to the west and a door to the south.".to_string(),
                exits: vec![
                    Exit {
                        direction: Direction::W,
                        target: 0,
                        locked: false,
                        key: String::from(""),
                    },
                    Exit {
                        direction: Direction::S,
                        target: 3,
                        locked: false,
                        key: String::from(""),
                    },
                ],
                interactables: vec![],
                items: vec![],
            },
            Room {
                description: "You find yourself in a room. There is a door to the north. A key is here.".to_string(),
                exits: vec![
                    Exit {
                        direction: Direction::N,
                        target: 0,
                        locked: false,
                        key: String::from(""),
                    },
                ],
                interactables: vec![],
                items: vec![],
            },
            Room {
                description: "You find yourself in a room. There is a door to the north. The door to the south is locked.".to_string(),
                exits: vec![
                    Exit {
                        direction: Direction::N,
                        target: 1,
                        locked: false,
                        key: String::from(""),
                    },
                    Exit {
                        direction: Direction::S,
                        target: 4,
                        locked: true,
                        key: String::from(""),
                    },
                ],
                interactables: vec![],
                items: vec![],
            },
            Room {
                description: "Dungeon exit".to_string(),
                exits: vec![],
                interactables: vec![],
                items: vec![],
    }
        ];
    let mut movement: Option<String> = None;
    let mut current_room = rooms.first();
    let mut parsed_input = Input {
        ..Default::default()
    };

    let item_vec = get_item_vec();
    let inventory_map: HashMap<_, _> = item_vec.into_iter().collect();

    while movement == None {
        println!("{}", current_room.unwrap().description);
        println!("\nWhat do you do?\n");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .ok()
            .expect("Failed to read line");

        let mut user_input = input.split_whitespace().peekable();

        // transform input to parsed form

        let first_command = user_input.next().unwrap();

        if !is_legal_command(first_command) {
            println!("{} is not a legal command\n", first_command);
            continue;
        };

        parsed_input.intent = first_command.to_string();
        for word in user_input {
            if parsed_input.object_noun == "" {
                if inventory_map.contains_key(word) {
                    parsed_input.object_noun = word.to_string();
                }
            }
        }

        println!("{:?}", parsed_input);
    }
}
