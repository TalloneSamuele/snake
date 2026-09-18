use crate::coordinate::Coordinate;
use crate::map::Map;
use rand::random_range;
#[derive(Clone)]
pub struct Food {
    symbol: String,
    position: Coordinate,
}

impl Food {
    pub fn random_positioned(symbol: String) -> Food {
        let random_coordinate: Coordinate = Coordinate::from(
            random_range(0..Map::DEFAULT_MAP_LENGTH),
            random_range(0..Map::DEFAULT_MAP_HIGHT),
        );

        return Food {
            symbol,
            position: random_coordinate,
        };
    }

    pub fn get_position(&self) -> Coordinate {
        return self.position.clone();
    }

    pub fn get_symbol(&self) -> String {
        return self.symbol.clone();
    }
}
