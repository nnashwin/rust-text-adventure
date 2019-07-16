use std::collections::HashMap;
use std::io;

#[derive(PartialEq)]
enum Command {
    Go(Direction),
    Unlock(Direction),
    Interact(String),
}

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

const LEGAL_COMMANDS: &'static [&'static str] = &[
    "go", "grab", "move", "pickup", "bite", "hit", "destroy", "shoot", "charge", "attack", "run",
    "jump", "climb",
];

fn is_legal_command<'a>(command_input: &'a str, legal_commands: &[&str]) -> Option<&'a str> {
    if legal_commands.iter().position(|&x| x == command_input) != None {
        Some(command_input)
    } else {
        None
    }
}

fn is_number(input: &str) -> bool {
    return input.parse::<i32>().is_ok();
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
    command: String,
    number: i32,
    object_noun: String,
}

struct Item {
    name: String,
    description: String,
    weight: i32,
}

struct Room {
    description: String,
    interactables: Vec<String>,
    items: Vec<Item>,
    exits: Vec<Exit>,
}

fn main() {
    let mut inventory_map = HashMap::new();
    inventory_map.insert("helmet", 0);
    inventory_map.insert("buster", 0);
    inventory_map.insert("boots", 0);
    inventory_map.insert("lab2_key", 0);
    inventory_map.insert("heat_armor", 0);

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
                interactables: vec!["exit_s".to_string()],
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
                interactables: vec!["exit_s".to_string()],
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
                interactables: vec!["exit_s".to_string()],
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
                interactables: vec!["exit_s".to_string()],
                items: vec![],
            },
            Room {
                description: "Dungeon exit".to_string(),
                exits: vec![],
                interactables: vec!["exit_s".to_string()],
                items: vec![],
    }
        ];
    let mut command: Option<String> = None;
    let mut current_room = rooms.first();
    let mut parsed_input = Input {
        ..Default::default()
    };

    while command == None {
        println!("{}", current_room.unwrap().description);
        println!("\nWhat do you do?\n");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .ok()
            .expect("Failed to read line");

        let mut user_input = input.split_whitespace().peekable();

        let first_command = user_input.next().unwrap();

        let predicate = match is_legal_command(first_command, LEGAL_COMMANDS) {
            Some(x) => x,
            None => "invalid command",
        };

        if predicate == "invalid command" {
            println!("'{}' is an invalid command\n", first_command);
            continue;
        }

        parsed_input.command = first_command.to_string();

        for word in user_input {
            println!("word {}, is_number {}", word, is_number(word));
            if is_number(word) {
                println!("The word is a digit")
            }
        }

        println!("{:?}", parsed_input);
    }
}
