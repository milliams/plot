use std::io::{self, BufRead};

fn main() {
    average();
}

fn average() {
    let stdin = io::stdin();
    let mut total = 0.0;
    let mut length = 0;
    for line in stdin.lock().lines() {
        let line_text = match line {
            Ok(line) => line,
            Err(err) => panic!("IO error: {}", err),
        };
        length += 1;
        total += line_text.parse::<f64>().unwrap();
    }
    println!("{}", total/length as f64);
}
