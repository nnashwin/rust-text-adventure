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
    "equip" => Intent::EQUIP,
    "examine" => Intent::EXAMINE,
    "pickup" => Intent::INVENTORY,
    "take" => Intent::INVENTORY,
    "grab" => Intent::INVENTORY,
    "push" => Intent::INTERACT,
    "touch" => Intent::INTERACT,
    "exit" => Intent::MOVEMENT,
    "go" => Intent::MOVEMENT,
    "move" => Intent::MOVEMENT,
    "run" => Intent::MOVEMENT,
    "walk" => Intent::MOVEMENT,
    "jump" => Intent::ELEVATE,
    "climb" => Intent::ELEVATE,
    "show" => Intent::LIST_INVENTORY,
    "list" => Intent::LIST_INVENTORY,
};

pub fn parse_command(command: &str) -> Option<Intent> {
    LEGAL_COMMANDS.get(command).cloned()
}
