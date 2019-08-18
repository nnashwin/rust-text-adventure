extern crate phf;
use phf::phf_map;

#[derive(Clone, Debug, PartialEq)]
pub enum Direction {
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

pub fn text_to_direction(text: &str) -> Option<Direction> {
    DIRECTION_MAPPINGS.get(text).cloned()
}

pub fn is_direction(direction: &str) -> bool {
    DIRECTION_MAPPINGS.contains_key(direction)
}
