use crate::coordinate::Coordinate;
use crate::direction::Direction;
use crate::food::Food;
use crate::snake::Snake;
use crate::tile::Tile;

#[derive(Clone)]
pub struct Map {
    length: usize,
    height: usize,
    grid: Vec<Vec<Tile>>,
}

impl Map {
    pub const DEFAULT_MAP_LENGTH: usize = 20;
    pub const DEFAULT_MAP_HIGHT: usize = 40;
    pub fn set(&mut self, tile: Tile, coord: Coordinate) {
        for i in 0..self.length {
            for j in 0..self.height {
                if i == coord.get_x().to_owned() && j == coord.get_y().to_owned() {
                    self.grid[i][j] = tile.clone();
                }
            }
        }
    }
    pub fn to_string(&self, snake: &Snake, food: &Vec<Food>) -> String {
        let mut out: String = String::new();
        for i in 0..self.length {
            for j in 0..self.height {
                if i == snake.get_head().get_x().to_owned()
                    && j == snake.get_head().get_y().to_owned()
                {
                    out += &(snake.get_char().to_string());
                    continue;
                }
                for k in 0..snake.get_body().len() {
                    if i == snake.get_body()[k].get_x().to_owned()
                        && j == snake.get_body()[k].get_y().to_owned()
                    {
                        out += &(snake.get_char().to_string());
                        continue;
                    }
                }

                for k in 0..food.len() {
                    if i == food[k].get_position().get_x().to_owned()
                        && j == food[k].get_position().get_y().to_owned()
                    {
                        out += &(food[k].get_symbol().to_string());
                        continue;
                    }
                }

                out += &(self.grid[j][i].to_string());
            }
            out += "\n";
        }
        return out;
    }
    pub fn default() -> Map {
        let mut grid: Vec<Vec<Tile>> =
            vec![vec![Tile::empty(); Map::DEFAULT_MAP_LENGTH]; Map::DEFAULT_MAP_HIGHT];
        for i in 0..Map::DEFAULT_MAP_HIGHT {
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
