use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        match line {
            Ok(line) => println!("{}", line),
            Err(err) => panic!("IO error: {}", err),
        }
    }
}
