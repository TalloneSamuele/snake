use crate::Direction;
use crate::coordinate::Coordinate;
const DEFAULT_CHAR: &str = "o";

#[derive(Clone)]
pub struct Snake {
    head: Coordinate,
    body: Vec<Coordinate>,
    char: String,
}
impl Snake {
    pub fn move_down(&mut self) {
        self.head.increase_x(1);
    }
    pub fn safe_move_down(&mut self) {
        self.head.safe_increase_x(1);
    }
    pub fn move_left(&mut self) {
        self.head.decrease_y(1);
    }
    pub fn safe_move_left(&mut self) {
        self.head.safe_decrease_y(1);
    }
    pub fn move_right(&mut self) {
        self.head.increase_y(1);
    }
    pub fn safe_move_right(&mut self) {
        self.head.safe_increase_y(1);
    }
    pub fn move_up(&mut self) {
        self.head.decrease_x(1);
    }
    pub fn safe_move_up(&mut self) {
        self.head.safe_decrease_x(1);
    }
    pub fn get_head(&self) -> Coordinate {
        return self.head.clone();
    }
    pub fn get_body(&self) -> Vec<Coordinate> {
        return self.body.clone();
    }
    pub fn get_char(&self) -> String {
        return self.char.clone();
    }

    pub fn new(head: Coordinate, body: Vec<Coordinate>) -> Snake {
        return Snake {
            head,
            body,
            char: DEFAULT_CHAR.to_string(),
        };
    }

    pub fn safe_move(&mut self, d: &Direction) {
        match d {
            Direction::DOWN => {
                self.safe_move_down();
            }
            Direction::LEFT => {
                self.safe_move_left();
            }
            Direction::RIGHT => {
                self.safe_move_right();
            }
            Direction::UP => {
                self.safe_move_up();
            }
            Direction::NULL => {}
        }
    }
}
