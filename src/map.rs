use crate::snake::Snake;

const DEFAULT_MAP_LENGTH: usize = 8;
const DEFAULT_MAP_HIGHT: usize = 8;
#[derive(Clone)]
pub struct Map {
    length: usize,
    height: usize,
    grid: Vec<Vec<Tile>>,
}

impl Map {
    pub fn set(&mut self, tile: Tile, coord: Coordinate) {
        for i in 0..self.length {
            for j in 0..self.height {
                if i == coord.x && j == coord.y {
                    self.grid[i][j] = tile.clone();
                }
            }
        }
    }
    pub fn set_player(&mut self, snake: &Snake) {
        for i in 0..self.length {
            for j in 0..self.height {
                if i == snake.get_head().x && j == snake.get_head().y {
                    self.grid[i][j].name = snake.get_char().to_string();
                } else {
                    self.grid[i][j] = Tile::default();
                }
            }
        }
    }
    pub fn to_string(&mut self, snake: &Snake) -> String {
        let mut out: String = String::new();
        for i in 0..self.length {
            for j in 0..self.height {
                if i == snake.get_head().x && j == snake.get_head().y {
                    out += &(snake.get_char().to_string());
                    continue;
                }
                out += &(self.grid[i][j].to_string());
            }
            out += "\n";
        }
        return out;
    }
    pub fn default() -> Map {
        let mut grid: Vec<Vec<Tile>> =
            vec![vec![Tile::empty(); DEFAULT_MAP_LENGTH]; DEFAULT_MAP_HIGHT];
        for i in 0..DEFAULT_MAP_LENGTH {
            for j in 0..DEFAULT_MAP_LENGTH {
                grid[i][j] = Tile::default();
            }
        }
        return Map {
            length: DEFAULT_MAP_LENGTH,
            height: DEFAULT_MAP_HIGHT,
            grid: grid,
        };
    }
}

#[derive(Clone)]
pub struct Coordinate {
    x: usize,
    y: usize,
}
impl Coordinate {
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
        if self.x == DEFAULT_MAP_LENGTH - 1 {
            return;
        }
        self.x += delta;
    }
    pub fn safe_increase_y(&mut self, delta: usize) {
        if self.y == DEFAULT_MAP_HIGHT - 1 {
            return;
        }
        self.y += delta;
    }
}
#[derive(Clone)]
pub struct Tile {
    name: String,
}
impl Tile {
    pub fn print(&self) {
        print!("{}", self.name)
    }
    pub fn to_string(&self) -> String {
        return self.name.to_string();
    }
    pub fn default() -> Tile {
        return Tile {
            name: "-".to_string(),
        };
    }
    pub fn empty() -> Tile {
        return Tile {
            name: "debug".to_string(),
        };
    }
    pub fn from(str: &str) -> Tile {
        return Tile {
            name: str.to_string(),
        };
    }
}
