use crate::map::Map;
#[derive(Clone)]
pub struct Coordinate {
    x: usize,
    y: usize,
}
impl Coordinate {
    pub fn get_x(&self) -> &usize {
        return &self.x;
    }
    pub fn get_y(&self) -> &usize {
        return &self.y;
    }

    pub fn to_string(self) -> String {
        return self.x.to_string() + " " + &self.y.to_string();
    }

    pub const fn from(x: usize, y: usize) -> Coordinate {
        return Coordinate { x, y };
    }
    pub fn decrease_x(&mut self, delta: usize) {
        self.x -= delta;
    }
    pub fn decrease_y(&mut self, delta: usize) {
        self.y -= delta;
    }
    pub fn increase_x(&mut self, delta: usize) {
        self.x += delta;
    }
    pub fn increase_y(&mut self, delta: usize) {
        self.y += delta;
    }
    pub fn safe_decrease_x(&mut self, delta: usize) {
        if self.x == 0 {
            return;
        }
        self.x -= delta;
    }
    pub fn safe_decrease_y(&mut self, delta: usize) {
        if self.y == 0 {
            return;
        }
        self.y -= delta;
    }
    pub fn safe_increase_x(&mut self, delta: usize) {
        if self.x == Map::DEFAULT_MAP_LENGTH - 1 {
            return;
        }
        self.x += delta;
    }
    pub fn safe_increase_y(&mut self, delta: usize) {
        if self.y == Map::DEFAULT_MAP_HIGHT - 1 {
            return;
        }
        self.y += delta;
    }
}
