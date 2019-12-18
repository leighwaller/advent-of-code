use std::fs::{File};
use std::io::Read;

fn main() {
    let data = read_input("data/input.csv");
    for command in data {
        println!("Processing: {}", command);
    }
}

struct Command {
    direction: Direction,
    distance: u16
}

enum Direction {
    Left,
    Right,
    Up,
    Down
}

fn parse_direction(c: char) -> Direction {
    return match c {
        'L' => Direction::Left,
        'R' => Direction::Right,
        'U' => Direction::Up,
        'D' => Direction::Down,
        _ => panic!("Unknown direction code: {}", c)
    }
}

fn read_input(path: &str) -> Vec<String> {
    // assumes only one line of input
    let mut line = String::new();
    File::open(path).unwrap().read_to_string(&mut line);
    return line.trim().split(",").map(str::to_string).collect();
}
