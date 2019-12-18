use std::fs::File;
use std::io::{Read};

const OP_CODE_ADD: usize = 1;
const OP_CODE_MULTIPLY: usize = 2;
const OP_CODE_EXIT: usize = 99;

fn main() {
    let mut data = read_input("data/input.csv");
    let mut pos = 0;
    loop {
        println!("processing op: {} at pos: {}", data[pos], pos);
        let x = data[pos + 1] as usize;
        let y = data[pos + 2] as usize;
        let z = data[pos + 3] as usize;

        match data[pos] {
            OP_CODE_ADD => {
                data[z] = x + y;
            }
            OP_CODE_MULTIPLY => {
                data[z] = x * y;
            }
            OP_CODE_EXIT => break,
            _ => println!("Unknown op code: {}", data[pos])
        }
        pos += 4;
    }
    println!("Value at 0: {}", data[0]);
}

fn read_input(path: &str) -> Vec<usize> {
    // assumes only one line of input
    let mut line = String::new();
    File::open(path).unwrap().read_to_string(&mut line);
    return line.trim()
        .split(",")
        .map(str::parse::<usize>)
        .map(Result::unwrap)
        .collect::<Vec<usize>>();
}
