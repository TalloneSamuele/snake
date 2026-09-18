#[derive(Clone)]
pub struct Tile {
    name: String,
}

impl Tile {
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
