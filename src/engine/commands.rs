extern crate phf;
use phf::phf_map;

#[derive(Clone, Debug, PartialEq)]
pub enum Intent {
    ATTACK,
    CHARGE,
    ELEVATE,
    EQUIP,
    EXAMINE,
    INVENTORY,
    INTERACT,
    LIST_INVENTORY,
    MOVEMENT,
    USE,
    NONE,
}

impl Default for Intent {
    fn default() -> Self {
        Intent::NONE
    }
}

pub static LEGAL_COMMANDS: phf::Map<&'static str, Intent> = phf_map! {
    "bite" => Intent::ATTACK,
    "hit" => Intent::ATTACK,
    "destroy" => Intent::ATTACK,
    "shoot" => Intent::ATTACK,
    "attack" => Intent::ATTACK,
    "charge" => Intent::CHARGE,
    "jump" => Intent::ELEVATE,
    "climb" => Intent::ELEVATE,
    "equip" => Intent::EQUIP,
    "examine" => Intent::EXAMINE,
    "pickup" => Intent::INVENTORY,
    "take" => Intent::INVENTORY,
    "grab" => Intent::INVENTORY,
    "push" => Intent::INTERACT,
    "touch" => Intent::INTERACT,
    "show" => Intent::LIST_INVENTORY,
    "list" => Intent::LIST_INVENTORY,
    "exit" => Intent::MOVEMENT,
    "go" => Intent::MOVEMENT,
    "move" => Intent::MOVEMENT,
    "run" => Intent::MOVEMENT,
    "walk" => Intent::MOVEMENT,
    "use" => Intent::USE,
};

pub fn determine_intent(command: &str) -> Option<Intent> {
    LEGAL_COMMANDS.get(command).cloned()
}

pub fn is_legal_command(command: &str) -> bool {
    LEGAL_COMMANDS.contains_key(command)
}
