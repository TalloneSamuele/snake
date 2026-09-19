#[derive(Clone, PartialEq)]

pub enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
    NULL,
}
impl Direction {
    pub fn to_string(self) -> String {
        match self {
            Direction::DOWN => {
                return "DOWN".to_string();
            }
            Direction::UP => {
                return "UP".to_string();
            }
            Direction::LEFT => {
                return "LEFT".to_string();
            }
            Direction::RIGHT => {
                return "RIGHT".to_string();
            }
            Direction::NULL => {
                return "NULL".to_string();
            }
        }
    }

    pub fn opposite(self) -> Direction {
        match self {
            Direction::DOWN => {
                return Direction::UP;
            }
            Direction::UP => {
                return Direction::DOWN;
            }
            Direction::LEFT => {
                return Direction::RIGHT;
            }
            Direction::RIGHT => {
                return Direction::LEFT;
            }
            Direction::NULL => {
                return Direction::NULL;
            }
        }
    }
}
