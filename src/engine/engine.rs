use std::collections::HashMap;
use std::io;

#[path = "commands.rs"]
mod commands;

#[path = "direction.rs"]
mod direction;

#[path = "examine.rs"]
mod examine;

#[path = "item.rs"]
mod item;

use commands::*;
use direction::*;
use examine::*;
use item::*;

#[derive(Clone, Debug)]
struct Exit {
    direction: Direction,
    target: usize,
    locked: bool,
    key: String,
}

#[derive(Clone, Debug)]
pub struct GameState {
    pub current_room_idx: usize,
    pub inventory: HashMap<&'static str, Item>,
    pub sys_message: String,
    pub rooms: Vec<Room>,
}

#[derive(Debug, Default)]
struct Input {
    intent: Intent,
    is_direction: bool,
    is_interactable: bool,
    is_item: bool,
    object_noun: String,
}

#[derive(Clone, Debug)]
struct Interactable {
    name: String,
    interaction_description: &'static str,
    before_interaction_description: &'static str,
    after_interaction_description: &'static str,
    interacted: bool,
}

impl Interactable {
    fn interact(&mut self) {
        self.interacted = true
    }
}

impl Examine for Interactable {
    fn examine(&self) -> &'static str {
        if self.interacted {
            self.after_interaction_description
        } else {
            self.before_interaction_description
        }
    }
}

#[derive(Clone, Debug)]
pub struct Room {
    description: String,
    interactables: Vec<Interactable>,
    items: Vec<&'static str>,
    exits: Vec<Exit>,
}

impl Room {
    pub fn get_description(&self) -> &str {
        &self.description
    }
    fn is_escape(&self) -> bool {
        self.exits.len() == 0
    }
}

pub fn start_game() -> GameState {
    let mut rooms = vec![
            Room {
                description: "You find yourself in a room. There is a door to the south and a door to the east. A stone sits in the far corner of the room to your west".to_string(),
                exits: vec![
                    Exit {
                        direction: Direction::S,
                        target: 2,
                        locked: false,
                        key: String::from(""),
                    },
                    Exit {
                        direction: Direction::E,
                        target: 1,
                        locked: false,
                        key: String::from(""),
                    },
                ],
                interactables: vec![Interactable{name: "stone".to_string(), interaction_description: "The stone falls to the floor", before_interaction_description: "You see a stone sitting in between two logs", after_interaction_description: "The stone rolled onto the floor and has revealed a secret passageway.", interacted: false}],
                items: vec![],
            },
            Room {
                description: "You find yourself in a room. There is a door to the west and a door to the south. You notice a small crevice in the corner.  The room with the helmet".to_string(),
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
                items: vec!["helmet"],
            },
            Room {
                description: "You find yourself in a room. There is a door to the north".to_string(),
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
                        locked: false,
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
    let mut current_room_idx = 0;

    let mut INVENTORY = create_inventory();

    return GameState {
        current_room_idx: 0,
        inventory: create_inventory(),
        rooms: rooms,
        sys_message: "".to_string(),
    };
}

pub fn update(mut prev_state: GameState, input: String) -> GameState {
    prev_state.current_room_idx;
    // println!("{}", prev_state.rooms.room.description);
    // println!("\nWhat do you do?\n");

    let mut parsed_input = Input {
        ..Default::default()
    };

    let mut new_game_state = prev_state.clone();

    let room = &mut prev_state.rooms[prev_state.current_room_idx];
    let mut user_inventory = prev_state.inventory;

    let mut user_input = input.split_whitespace().peekable();
    let first_command = user_input.next().unwrap();

    if !is_legal_command(first_command) {
        // If the command is not valid, we do not need to parse the rest of the string input
        new_game_state.sys_message = format!("{} is not a legal command\n", first_command);
        return new_game_state;
    };

    parsed_input.intent = determine_intent(first_command).unwrap();

    for word in user_input {
        let lowercase_word = word.to_lowercase();
        if parsed_input.object_noun == "" {
            if lowercase_word == "inventory" {
                parsed_input.object_noun = lowercase_word;
                continue;
            }

            if is_direction(lowercase_word.as_str()) {
                parsed_input.object_noun = lowercase_word;
                parsed_input.is_direction = true;
                continue;
            }

            if user_inventory.contains_key(lowercase_word.as_str()) {
                parsed_input.object_noun = lowercase_word;
                parsed_input.is_item = true;
                continue;
            }

            if room.interactables.iter().any(|x| x.name == lowercase_word) {
                parsed_input.object_noun = lowercase_word;
                parsed_input.is_interactable = true;
                continue;
            }
        }
    }

    match parsed_input.intent {
        Intent::ATTACK => println!("attack"),
        Intent::CHARGE => println!("charge"),
        Intent::ELEVATE => println!("elevate"),
        Intent::EXAMINE => {
            if parsed_input.is_interactable {
                let description = room
                    .interactables
                    .iter()
                    .find(|x| x.name == parsed_input.object_noun)
                    .unwrap()
                    .examine();

                new_game_state.sys_message = description.to_string();
            } else if parsed_input.is_item {
                let description = user_inventory
                    .get::<str>(&parsed_input.object_noun)
                    .unwrap()
                    .get_description();
                new_game_state.sys_message = description.to_string();
            }
        }
        Intent::INTERACT => {
            if parsed_input.is_interactable {
                for i in 0..room.interactables.len() {
                    if room.interactables[i].name == parsed_input.object_noun {
                        room.interactables[i].interact();
                        new_game_state.sys_message =
                            String::from(room.interactables[i].interaction_description);
                        continue;
                    }
                }
            }
        }
        Intent::INVENTORY => {
            let key = &parsed_input.object_noun;
            if parsed_input.is_item {
                user_inventory.get_mut::<str>(key).unwrap().to_inventory();
                new_game_state.sys_message = format!(
                    "You have picked up a {}",
                    user_inventory
                        .get_mut::<str>(key)
                        .unwrap()
                        .get_name()
                        .to_string()
                );
            }
        }
        Intent::LIST_INVENTORY => {
            let mut inventory_message: String = "".to_owned();
            inventory_message.push_str("Your inventory:\n");
            for item in user_inventory.values() {
                if item.get_location() == &ItemState::Equipped {
                    inventory_message.push_str(&format!(
                        "{}: {}\n",
                        item.get_name(),
                        item.get_description()
                    ));
                }
            }

            new_game_state.sys_message = inventory_message;
        }
        Intent::MOVEMENT => {
            if parsed_input.is_direction {
                let direction: Direction = text_to_direction(&parsed_input.object_noun).unwrap();

                let exit: Option<&Exit> = room.exits.iter().find(|&x| x.direction == direction);

                if exit.is_none() {
                    new_game_state.sys_message =
                        format!("There is no exit leaving {}", parsed_input.object_noun);
                } else if parsed_input.is_direction && exit.is_some() {
                    new_game_state.current_room_idx = exit.unwrap().target;
                    new_game_state.sys_message = new_game_state.rooms
                        [new_game_state.current_room_idx]
                        .description
                        .to_string();
                }
            } else {
                new_game_state.sys_message =
                    format!("You can not move to {}", parsed_input.object_noun);
            }
        }
        Intent::USE => new_game_state.sys_message = format!("use"),
        _ => new_game_state.sys_message = format!("You didn't choose an appropriate command"),
    }

    return new_game_state;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interact() {
        let new_inter = &mut Interactable {
            name: "stone".to_string(),
            interaction_description: "The stone rolls onto the floor",
            before_interaction_description: "You see a stone sitting in between two logs",
            after_interaction_description: "The stone is sitting on the floor",
            interacted: false,
        };
        assert_eq!(new_inter.interacted, false);
        new_inter.interact();

        assert_eq!(new_inter.interacted, true);
    }
}
