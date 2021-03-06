use std::fs::File;
use std::io::{BufReader, BufRead, Lines};
use std::collections::HashSet;
use std::path::Path;

fn main() {
    process_lines(read_file("data/input.csv").lines());
}

fn process_lines(mut lines: Lines<BufReader<File>>) {
    // problem expects exactly 2 lines (wires) so this is crude but ok
    let wire1 = lines.next().unwrap().unwrap();
    let wire2 = lines.next().unwrap().unwrap();
    let distance = shortest_distance(wire1.as_str(), wire2.as_str());
    println!("Distance: {}", distance);
}

fn shortest_distance(path1: &str, path2: &str) -> i16 {
    println!("Calculating closest intersecting distance of {} and {}", path1, path2);
    let points = map_points(path1);
    let points2 = map_points(path2);
    let intersections = find_intersections(points, points2);
    return compute_shortest_manhattan(&intersections);
}

fn compute_shortest_manhattan(points: &HashSet<Point>) -> i16 {
    let home = Point::central();
    let dist = points.iter()
        .map(|p| apply_manhattan(&p, &home))
        .min()
        .unwrap();
    println!("Manhattan distance: {}", dist);
    return dist;
}

fn apply_manhattan(a: &Point, b: &Point) -> i16 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

fn find_intersections(points: HashSet<Point>, points2: HashSet<Point>) -> HashSet<Point> {
    let mut intersections: HashSet<Point> = HashSet::new();
    for point in points {
        if points2.contains(&point) && !point.is_home() {
            intersections.insert(point);
        }
    }
    intersections
}

fn map_points(path: &str) -> HashSet<Point> {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    let mut points: HashSet<Point> = HashSet::new();
    for input in parse_line(path) {
        let (direction, distance) = input.split_at(1);
        let command = Command::new(direction, distance);
        for _ in 0..command.distance {
            match command.direction {
                Direction::Left => x -= 1,
                Direction::Right => x += 1,
                Direction::Down => y -= 1,
                Direction::Up => y += 1
            }
            points.insert(Point::new(x, y));
        }
    }
    points
}

#[derive(Debug, PartialEq)]
struct Command {
    direction: Direction,
    distance: i16,
}

impl Command {
    fn new(direction: &str, distance: &str) -> Command {
        return Command {
            direction: Direction::new(direction),
            distance: distance.parse().unwrap(),
        };
    }
}

#[derive(Debug, PartialEq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
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

#[derive(Debug, Eq, PartialEq, Hash)]
struct Point {
    x: i16,
    y: i16,
}

impl Point {
    fn new(x: i16, y: i16) -> Point {
        Point { x, y }
    }

    fn central() -> Point {
        Point::new(0, 0)
    }

    fn is_home(&self) -> bool {
        self.eq(&Point::central())
    }
}

fn parse_line(line: &str) -> Vec<String> {
    return line.trim()
        .split(",")
        .map(str::to_string)
        .collect();
}

fn read_file(name: &str) -> BufReader<File> {
    let path = Path::new(name);
    println!("opening input file: {:?}", path);
    let file = File::open(&path).unwrap();
    return BufReader::new(file);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_calculate_distance_example1() {
        let path1 = "R75,D30,R83,U83,L12,D49,R71,U7,L72";
        let path2 = "U62,R66,U55,R34,D71,R55,D58,R83";

        let distance = shortest_distance(path1, path2);

        assert_eq!(distance, 159);
    }

    #[test]
    fn should_calculate_distance_example2() {
        let path1 = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51";
        let path2 = "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";

        let distance = shortest_distance(path1, path2);

        assert_eq!(distance, 135);
    }
}
