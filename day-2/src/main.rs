use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{Add, Mul};
use std::process::exit;

const OP_CODE_ADD: i32 = 1;
const OP_CODE_MULTIPLY: i32 = 2;
const OP_CODE_EXIT: i32 = 99;

fn main() {
    let file = read_file("data/input.csv");
    let mut data = parse_data(file);
    let mut pos = 0;
    loop {
        println!("processing op: {} at pos: {}", data[pos], pos);
        match data[pos] {
            OP_CODE_ADD => {
                let x = get_x(&data, pos);
                let y = get_y(&data, pos);
                let zidx = pos + 3;
                let z = x.add(y);
                println!("Setting value at position: {} to {}", zidx, z);
                data[zidx] = z;
            }
            OP_CODE_MULTIPLY => {
                let x = get_x(&data, pos);
                let y = get_y(&data, pos);
                let zidx = pos + 3;
                let z = x.mul(y);
                println!("Setting value at position: {} to {}", zidx, z);
                data[zidx] = z;
            }
            OP_CODE_EXIT => break,
            _ => println!("Unknown op code: {}", data[pos])
        }
        pos = pos + 4;
    }
    println!("Value at 0: {}", data[0]);
}

fn get_x(data: &Vec<i32>, pos: usize) -> i32 {
    data[pos + 1]
}

fn get_y(data: &Vec<i32>, pos: usize) -> i32 {
    data[pos + 2]
}

fn parse_data(file: BufReader<File>) -> Vec<i32> {
    let mut vals: Vec<i32> = vec![];
    // should really only be one row in this case
    for result in file.lines() {
        let line = result.unwrap();
        let fields: Vec<&str> = line.split_terminator(",").collect();
        parse_fields(&mut vals, fields)
    }
    return vals;
}

fn parse_fields(vals: &mut Vec<i32>, fields: Vec<&str>) -> () {
    for field in fields {
        vals.push(field.parse().unwrap());
    }
}

fn read_file(path: &str) -> BufReader<File> {
    let file = File::open(path).unwrap();
    return BufReader::new(file);
}