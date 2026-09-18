#[allow(dead_code)]
use std::io::Write;
use std::io::{self, Read};
use std::process::Command;
use std::sync::Once;
use std::thread;
use std::time::Duration;

mod map;
use map::Map;

mod snake;
use snake::Snake;

mod coordinate;
use coordinate::Coordinate;

mod tile;
use tile::Tile;

mod food;
use food::Food;

static INIT: Once = Once::new();

const STARTING_HEAD_POSITION: Coordinate =
    Coordinate::from(Map::DEFAULT_MAP_LENGTH / 2, Map::DEFAULT_MAP_HIGHT / 2);
const FRAME_TIME: u64 = 1 / 60 * 1000;

fn main() {
    let player_head: Coordinate = STARTING_HEAD_POSITION;
    let player_body: Vec<Coordinate> = Vec::new();

    let mut snake: Snake = Snake::new(player_head, player_body);
    let map: Map = Map::default();
    let mut food: Vec<Food> = Vec::new();

    loop {
        update(&mut snake, &map, &mut food);
        thread::sleep(Duration::from_millis(FRAME_TIME));
    }
}

fn update(snake: &mut Snake, map: &Map, food: &mut Vec<Food>) {
    print!(
        "\x1b[2J\x1b[H{}{}",
        map.to_string(&snake, &food),
        snake.get_head().to_string(),
    );
    io::stdout().flush().unwrap();

    listen_keys(snake);
    generate_food(food);
    if check_position(snake, food) {
        food.remove(get_position(snake, food));
    }
}

fn listen_keys(snake: &mut Snake) {
    INIT.call_once(|| {
        Command::new("stty")
            .args(["-icanon", "-echo", "min", "0", "time", "0"])
            .status()
            .unwrap();
    });

    let mut buffer = [0u8; 1];

    if io::stdin().read(&mut buffer).unwrap() > 0 {
        match buffer[0] {
            b'w' => {
                snake.safe_move_up();
            }

            b'a' => {
                snake.safe_move_left();
            }

            b's' => {
                snake.safe_move_down();
            }

            b'd' => {
                snake.safe_move_right();
            }

            b'q' => {
                Command::new("stty").arg("sane").status().unwrap();

                std::process::exit(0);
            }

            _ => {}
        }
    }
}

fn generate_food(food: &mut Vec<Food>) {
    if food.len() < 2 {
        food.push(Food::random_positioned("@".to_string()));
    }
}

fn check_position(snake: &Snake, food: &Vec<Food>) -> bool {
    for i in 0..food.len() {
        if food[i].get_position().equals(snake.get_head()) {
            return true;
        }
    }
    return false;
}

fn get_position(snake: &Snake, food: &Vec<Food>) -> usize {
    for i in 0..food.len() {
        if food[i].get_position().equals(snake.get_head()) {
            return i;
        }
    }
    return 9999;
}

fn get_user_input(hint: &str) -> String {
    use std::io::{Write, stdin, stdout};
    let mut s = String::new();
    print!("{}", hint);
    let _ = stdout().flush();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }
    return s;
}
