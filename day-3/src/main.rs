use std::fs::File;
use std::io::{BufReader, BufRead, Lines};

fn main() {
    process(read_file("data/input.csv").lines())
}

fn process(lines: Lines<BufReader<File>>) {
    for line in lines {
        let data = parse_line(line.unwrap());
        process_line(data)
    }
}

fn process_line(data: Vec<String>) {
    for input in data {
        println!("Processing: {}", input);
        let (direction, distance) = input.split_at(1);
        let command = Command::new(direction, distance);
        // todo
    }
}

#[derive(Debug)]
struct Command {
    direction: Direction,
    distance: u16,
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Command {
    fn new(direction: &str, distance: &str) -> Command {
        return Command {
            direction: Direction::new(direction),
            distance: distance.parse().unwrap(),
        };
    }
}

impl Direction {
    fn new(direction: &str) -> Direction {
        let c = direction.chars().nth(0).unwrap();
        return match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            'U' => Direction::Up,
            'D' => Direction::Down,
            _ => panic!("Unknown direction code: {}", c)
        };
    }
}

fn parse_line(line: String) -> Vec<String> {
    return line.trim()
        .split(",")
        .map(str::to_string)
        .collect();
}

fn read_file(path: &str) -> BufReader<File> {
    let file = File::open(path).unwrap();
    return BufReader::new(file);
}
