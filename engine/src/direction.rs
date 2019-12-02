use serde_derive::{Serialize, Deserialize};
extern crate phf;
use phf::phf_map;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Direction {
    N,
    S,
    E,
    W,
    NE,
    NW,
    SE,
    SW,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_direction_invalid() {
        let expected = None;
        let actual = text_to_direction("Hamster");
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_to_direction_valid() {
        let expected = Some(Direction::N);
        let actual = text_to_direction("north");

        assert_eq!(expected, actual);
        assert_eq!(actual.unwrap(), Direction::N);
    }
}
