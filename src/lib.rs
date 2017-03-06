extern crate plotlib;

mod render;

use std::io::{self, BufRead};

pub struct HistogramConfig {
    pub nbins: u32,
}

/// Get a single column of data from stdin
///
/// For each line in the input, it tries to convert it to an `f64`.
fn get_single_column() -> Vec<f64> {
    let stdin = io::stdin();
    let mut data: Vec<f64> = vec![];
    for line in stdin.lock().lines() {
        let line_text = match line {
            Ok(line) => line,
            Err(err) => panic!("IO error: {}", err),
        };
        data.push(line_text.parse::<f64>()
            .expect(&format!("ERROR: Could not parse '{}' as an f64", line_text)));
    }
    data
}

fn get_two_columns() -> Vec<(f64, f64)> {
    let stdin = io::stdin();
    let mut data: Vec<(f64, f64)> = vec![];
    for line in stdin.lock().lines() {
        let line_text = match line {
            Ok(line) => line,
            Err(err) => panic!("IO error: {}", err),
        };
        let nums: Vec<_> = line_text.split_whitespace().collect();
        if nums.len() != 2 {
            panic!("Wrong number of args on line");
        }
        let a = nums[0];
        let b = nums[1];

        let a = a.parse::<f64>()
            .expect(&format!("ERROR: Could not parse '{}' as an f64", a));
        let b = b.parse::<f64>()
            .expect(&format!("ERROR: Could not parse '{}' as an f64", b));

        data.push((a, b));
    }
    data
}

pub fn hist(config: HistogramConfig) {
    let data = get_single_column();

    let h = plotlib::histogram::Histogram::from_vec(&data, config.nbins);

    render::draw_histogram(&h);
}

pub fn scatter() {
    let data = get_two_columns();

    let h = plotlib::scatter::Scatter::from_vec(&data);

    render::draw_scatter(&h);
}

pub fn average() {
    let stdin = io::stdin();
    let mut total = 0.0;
    let mut length = 0;
    for line in stdin.lock().lines() {
        let line_text = match line {
            Ok(line) => line,
            Err(err) => panic!("IO error: {}", err),
        };
        length += 1;
        total += line_text.parse::<f64>()
            .expect(&format!("ERROR: Could not parse '{}' as an f64", line_text));
    }

    println!("{}", total / length as f64);
}

pub fn stats() {
    let data = get_single_column();

    let max = data.iter().fold(-1. / 0., |a, &b| f64::max(a, b));
    let min = data.iter().fold(1. / 0., |a, &b| f64::min(a, b));
    let total: f64 = data.iter().sum();
    let average = total / data.len() as f64;
    let length = data.len();

    println!("    Max: {}", max);
    println!("    Min: {}", min);
    println!("Average: {}", average);
    println!(" Length: {}", length);
}
