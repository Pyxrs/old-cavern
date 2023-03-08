use glam::IVec3;

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub enum Direction {
    UP,
    DOWN,
    NORTH,
    WEST,
    SOUTH,
    EAST
}

impl Direction {
    pub fn get(id: u8) -> Direction {
        match id {
            0 => Direction::UP,
            1 => Direction::DOWN,
            2 => Direction::NORTH,
            3 => Direction::SOUTH,
            4 => Direction::WEST,
            5 => Direction::EAST,
            _ => panic!("Invalid ID!"),
        }
    }
    pub fn get_normal(&self) -> IVec3 {
        match self {
            Direction::UP => IVec3::new(0, 1, 0),
            Direction::DOWN => IVec3::new(0, -1, 0),
            Direction::NORTH => IVec3::new(0, 0, -1),
            Direction::SOUTH => IVec3::new(0, 0, 1),
            Direction::WEST => IVec3::new(1, 0, 0),
            Direction::EAST => IVec3::new(-1, 0, 0),
        }
    }
    pub fn get_id(&self) -> u8 {
        match self {
            Direction::UP => 0,
            Direction::DOWN => 1,
            Direction::NORTH => 2,
            Direction::SOUTH => 3,
            Direction::WEST => 4,
            Direction::EAST => 5,
        }
    }
    pub fn get_string(&self) -> String {
        match self {
            Direction::UP => String::from("up"),
            Direction::DOWN => String::from("down"),
            Direction::NORTH => String::from("north"),
            Direction::SOUTH => String::from("south"),
            Direction::WEST => String::from("west"),
            Direction::EAST => String::from("east"),
        }
    }
}