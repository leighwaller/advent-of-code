use std::fs::File;
use std::io::{BufReader, BufRead};
use math::round;
use std::ops::Add;

fn main() {
    let reader = read_file("data/input.txt");
    let mut total = 0;
    for line in reader.lines() {
        let mass = parse_line(line.unwrap());
        process_mass(&mut total, mass)
    }
    println!("\nTotal fuel required: {}", total);
}

fn process_mass(total: &mut i32, mut mass: i32) -> () {
    loop {
        let fuel = calc_fuel(mass);
        if fuel <= 0 {
            break;
        }
        println!("Calculated fuel requirement: {} for mass: {}", fuel, mass);
        *total = total.add(fuel);
        mass = fuel;
    }
}

fn parse_line(line: String) -> i32 {
    return line.trim().parse().unwrap();
}

fn read_file(path: &str) -> BufReader<File> {
    let file = File::open(path).unwrap();
    return BufReader::new(file);
}

fn calc_fuel(mass: i32) -> i32 {
    let scale = 4;
    let value = (mass / 3) as f64;
    return (round::floor(value, scale) - 2f64) as i32;
}
