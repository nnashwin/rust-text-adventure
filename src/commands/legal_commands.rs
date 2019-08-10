use phf::phf_map;

#[derive(Clone, Debug, PartialEq)]
pub enum Intent {
    MOVEMENT,
    USE,
    ATTACK,
    CHARGE,
    ELEVATE,
    INTERACT,
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
    "pickup" => Intent::INTERACT,
    "grab" => Intent::INTERACT,
    "touch" => Intent::INTERACT,
    "exit" => Intent::MOVEMENT,
    "go" => Intent::MOVEMENT,
    "move" => Intent::MOVEMENT,
    "run" => Intent::MOVEMENT,
    "walk" => Intent::MOVEMENT,
    "jump" => Intent::ELEVATE,
    "climb" => Intent::ELEVATE,
};

pub fn parse_command(command: &str) -> Option<Intent> {
    LEGAL_COMMANDS.get(command).cloned()
}
