use crate::coordinate::Coordinate;
use crate::snake::Snake;
use crate::tile::Tile;

#[derive(Clone)]
pub struct Map {
    length: usize,
    height: usize,
    grid: Vec<Vec<Tile>>,
}

impl Map {
    pub const DEFAULT_MAP_LENGTH: usize = 8;
    pub const DEFAULT_MAP_HIGHT: usize = 8;
    pub fn set(&mut self, tile: Tile, coord: Coordinate) {
        for i in 0..self.length {
            for j in 0..self.height {
                if i == coord.get_x().to_owned() && j == coord.get_y().to_owned() {
                    self.grid[i][j] = tile.clone();
                }
            }
        }
    }
    pub fn to_string(&mut self, snake: &Snake) -> String {
        let mut out: String = String::new();
        for i in 0..self.length {
            for j in 0..self.height {
                if i == snake.get_head().get_x().to_owned()
                    && j == snake.get_head().get_y().to_owned()
                {
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
            vec![vec![Tile::empty(); Map::DEFAULT_MAP_LENGTH]; Map::DEFAULT_MAP_HIGHT];
        for i in 0..Map::DEFAULT_MAP_LENGTH {
            for j in 0..Map::DEFAULT_MAP_LENGTH {
                grid[i][j] = Tile::default();
            }
        }
        return Map {
            length: Map::DEFAULT_MAP_LENGTH,
            height: Map::DEFAULT_MAP_HIGHT,
            grid: grid,
        };
    }
}
