//! A command-line plotting tool

extern crate clap;
use clap::{Arg, App, SubCommand};

use std::io::{self, BufRead};

mod histogram;
mod render;

fn main() {
    let matches = App::new("Command-line stats")
        .about("Does awesome things")
        .subcommand(SubCommand::with_name("average")
            .about("computes the average of the input stream"))
        .subcommand(SubCommand::with_name("hist").about("plots a histogram of the data"))
        .subcommand(SubCommand::with_name("stats").about("print some statistica about the data"))
        .get_matches();

    if matches.subcommand_matches("average").is_some() {
        average();
    }

    if matches.subcommand_matches("stats").is_some() {
        stats();
    }

    if matches.subcommand_matches("hist").is_some() {
        hist();
    }
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
        data.push(line_text.parse::<f64>().unwrap());
    }
    return data;
}

fn hist() {
    let data = get_single_column();

    let h = histogram::Histogram::from_vec(&data);

    render::draw_histogram(h);

    //println!("{:?}", h.bin_counts);
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

    println!("{}", total / length as f64);
}

fn stats() {
    let data = get_single_column();

    let max = data.iter().fold(-1. / 0., |a, &b| f64::max(a, b));
    let min = data.iter().fold(1. / 0., |a, &b| f64::min(a, b));
    let total: f64 = data.iter().sum();
    let average = total / data.len() as f64;

    println!("    Max: {}", max);
    println!("    Min: {}", min);
    println!("Average: {}", average);
}
