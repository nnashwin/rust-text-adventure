use std::collections::HashMap;
use std::io;

extern crate phf;

use phf::phf_map;

mod commands;

fn create_inventory() -> HashMap<&'static str, Item> {
    let mut map = HashMap::new();

    map.insert(
        "helmet",
        Item {
            name: "helmet".to_string(),
            description: "A blue helmet covered in dirt".to_string(),
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

#[derive(Debug)]
struct Command {
    intent: commands::legal_commands::Intent,
    target_room: Option<usize>,
    item: Option<Item>,
    interactable: Option<Interactable>,
}

fn check_command_optional(optional: Option<Command>) -> bool {
    match optional {
        Some(_command) => true,
        None => false,
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Direction {
    N,
    S,
    E,
    W,
    NE,
    NW,
    SE,
    SW,
    NONE,
}

impl Default for Direction {
    fn default() -> Self {
        Direction::NONE
    }
}

static DIRECTION_MAPPINGS: phf::Map<&'static str, Direction> = phf_map! {
    "north" => Direction::N,
    "south" => Direction::S,
    "east" => Direction::E,
    "west" => Direction::W,
    "northeast" => Direction::NE,
    "northwest" => Direction::NW,
    "southeast" => Direction::SE,
    "southwest" => Direction::SW,
};

fn text_to_direction(text: &str) -> Option<Direction> {
    DIRECTION_MAPPINGS.get(text).cloned()
}

fn is_direction(direction: &str) -> bool {
    DIRECTION_MAPPINGS.contains_key(direction)
}

fn is_legal_command(command: &str) -> bool {
    commands::legal_commands::LEGAL_COMMANDS.contains_key(command)
}

fn is_obj_noun<'a>(word: &'a str, item_map: &HashMap<String, Item>) -> bool {
    return item_map.contains_key(word);
}

#[derive(Debug)]
struct Exit {
    direction: Direction,
    target: usize,
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
    intent: commands::legal_commands::Intent,
    is_direction: bool,
    is_interactable: bool,
    is_item: bool,
    object_noun: String,
}

impl Input {
    fn reset_input(&mut self) {
        self.intent = commands::legal_commands::Intent::NONE;
        self.object_noun = "".to_string();
        self.is_interactable = false;
        self.is_item = false;
    }
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
    weight: usize,
    location: ItemState,
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

#[derive(Debug)]
struct Room {
    description: String,
    interactables: Vec<Interactable>,
    items: Vec<&'static str>,
    exits: Vec<Exit>,
}

impl Room {
    fn is_escape(&self) -> bool {
        self.exits.len() == 0
    }
}

fn main() {
    let mut rooms = vec![
            Room {
                description: "You find yourself in a room. There is a door to the south and a door to the east.".to_string(),
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
                interactables: vec![Interactable{name: "stone".to_string(), before_interaction_description: "You see a stone sitting in between two logs".to_string(), after_interaction_description: "The stone rolls onto the floor".to_string(), interacted: false}],
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
    let mut current_room = 0;

    let mut INVENTORY = create_inventory();

    while !rooms[current_room].is_escape() {
        current_room =
            enter(&mut INVENTORY, rooms.get_mut(current_room).unwrap()).unwrap_or(current_room);
    }

    println!("You have escaped the ruins.  Consider yourself lucky");
}

fn enter(INVENTORY: &mut HashMap<&'static str, Item>, room: &mut Room) -> Option<usize> {
    println!("{}", room.description);
    println!("\nWhat do you do?\n");

    let mut command: Option<Command> = None;
    let mut parsed_input = Input {
        ..Default::default()
    };

    while let None = command {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .ok()
            .expect("Failed to read line");

        let mut user_input = input.split_whitespace().peekable();

        // PARSE USER INPUT
        let first_command = user_input.next().unwrap();

        if !is_legal_command(first_command) {
            println!("{} is not a legal command\n", first_command);
            continue;
        };

        parsed_input.intent = commands::legal_commands::parse_command(first_command).unwrap();

        for word in user_input {
            let lowercase_word = word.to_lowercase();
            println!("lowercase_word: {}", lowercase_word);
            if parsed_input.object_noun == "" {
                if is_direction(lowercase_word.as_str()) {
                    parsed_input.object_noun = lowercase_word;
                    parsed_input.is_direction = true;
                    continue;
                }

                if INVENTORY.contains_key(lowercase_word.as_str()) {
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
            commands::legal_commands::Intent::ATTACK => println!("attack"),
            commands::legal_commands::Intent::CHARGE => println!("charge"),
            commands::legal_commands::Intent::ELEVATE => println!("elevate"),
            commands::legal_commands::Intent::INTERACT => {
                println!("{:?}", parsed_input);

                // command = if parsed_input.is_item {
                //     INVENTORY
                // }
            }
            commands::legal_commands::Intent::MOVEMENT => {
                let direction: Direction = text_to_direction(&parsed_input.object_noun).unwrap();

                let exit: Option<&Exit> = room.exits.iter().find(|&x| x.direction == direction);

                // Print out incorrect direction
                if parsed_input.is_direction {
                    if exit.is_none() {
                        println!("There is no exit leaving {}", parsed_input.object_noun);
                    } else if parsed_input.is_direction && exit.is_some() {
                        command = Some(Command {
                            intent: commands::legal_commands::Intent::MOVEMENT,
                            target_room: Some(exit.unwrap().target),
                            interactable: None,
                            item: None,
                        })
                    }
                }
            }
            commands::legal_commands::Intent::USE => println!("use"),
            _ => println!("You didn't choose an appropriate command"),
        }

        parsed_input.reset_input();
    }

    let unwrapped_command = command.unwrap();
    let is_movement = unwrapped_command.intent == commands::legal_commands::Intent::MOVEMENT;

    match is_movement {
        true => unwrapped_command.target_room,
        _ => None,
    }
}
