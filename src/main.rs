#[allow(dead_code)]
use std::io::Write;
use std::io::{self, Read};
use std::process::Command;
use std::sync::Once;
use std::thread;
use std::time::Duration;

mod map;
use map::Coordinate;
use map::Map;

mod snake;
use snake::Snake;

static INIT: Once = Once::new();

const STARTING_HEAD_POSITION: Coordinate = Coordinate::from(0, 0);
const FRAME_TIME: u64 = 1 / 60 * 1000;

fn main() {
    let player_name: String = get_user_input("Enter your name: ");
    let mut player_head: Coordinate = STARTING_HEAD_POSITION;
    let mut player_body: Vec<Coordinate> = Vec::new();

    let mut snake: Snake = Snake::new(player_name, player_head, player_body);
    let mut map: Map = Map::default();

    loop {
        update(&mut snake);
        print!(
            "\x1b[2J\x1b[H{}{}",
            map.to_string(&snake),
            snake.get_head().to_string(),
        );
        io::stdout().flush().unwrap();

        thread::sleep(Duration::from_millis(FRAME_TIME));
    }
}

fn update(snake: &mut Snake) {
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
