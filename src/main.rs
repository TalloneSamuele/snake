#[allow(dead_code)]
use std::io::Write;
use std::io::{self, Read};
use std::process::Command;
use std::sync::Once;
use std::thread;
use std::time::Duration;

mod tile;

mod map;
use map::Map;

mod snake;
use snake::Snake;

mod coordinate;
use coordinate::Coordinate;

mod food;
use food::Food;

mod direction;
use direction::Direction;

static INIT: Once = Once::new();

const STARTING_HEAD_POSITION: Coordinate =
    Coordinate::from(Map::DEFAULT_MAP_LENGTH / 2, Map::DEFAULT_MAP_HIGHT / 2);

const FPS: u32 = 5;

fn main() {
    let frame_time = Duration::from_secs(1) / FPS;

    let player_head: Coordinate = STARTING_HEAD_POSITION;
    let player_body: Vec<Coordinate> = Vec::new();

    let mut snake: Snake = Snake::new(player_head, player_body);
    let map: Map = Map::default();
    let mut food: Vec<Food> = Vec::new();
    let mut last_direction: Direction = Direction::NULL;

    loop {
        update(&mut snake, &map, &mut food, &mut last_direction);
        thread::sleep(frame_time);
    }
}

fn update(snake: &mut Snake, map: &Map, food: &mut Vec<Food>, last_direction: &mut Direction) {
    print!(
        "\x1b[2J\x1b[H{}{} {}",
        map.to_string(&snake, &food),
        snake.get_head().to_string(),
        last_direction.clone().to_string(),
    );
    io::stdout().flush().unwrap();
    let direction: Direction = listen_keys();
    update_player_position(snake, &direction, last_direction);
    generate_food(food);
    if check_position(snake, food) {
        position_matches(snake, food);
    }
}

fn listen_keys() -> Direction {
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
                return Direction::UP;
            }

            b'a' => {
                return Direction::LEFT;
            }

            b's' => {
                return Direction::DOWN;
            }

            b'd' => {
                return Direction::RIGHT;
            }

            b'q' => {
                Command::new("stty").arg("sane").status().unwrap();

                std::process::exit(0);
            }

            _ => {}
        }
    }
    return Direction::NULL;
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

fn position_matches(snake: &mut Snake, food: &mut Vec<Food>) {
    let index = get_matching_position_index(snake, food);

    let position = food[index].get_position();

    food.remove(index);
    snake.add_to_body(position);
}

fn get_matching_position_index(snake: &Snake, food: &Vec<Food>) -> usize {
    for i in 0..food.len() {
        if food[i].get_position().equals(snake.get_head()) {
            return i;
        }
    }
    return 99999;
}

fn update_player_position(
    snake: &mut Snake,
    direction: &Direction,
    mut last_direction: &mut Direction,
) {
    if direction.clone() == Direction::NULL {
        snake.safe_move(last_direction);
        snake.move_body(last_direction);
        return;
    }
    snake.safe_move(direction);
    *last_direction = direction.clone();
}
