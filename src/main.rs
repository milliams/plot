extern crate clap;
use clap::{Arg, App, SubCommand};

use std::io::{self, BufRead};

fn main() {
    let matches = App::new("Command-line stats")
        .about("Does awesome things")
        .subcommand(SubCommand::with_name("average")
            .about("computes the average of the input stream"))
        .subcommand(SubCommand::with_name("hist")
            .about("plots a histogram of the data"))
        .get_matches();

    if matches.subcommand_matches("average").is_some() {
        average();
    }

    if matches.subcommand_matches("hist").is_some() {
        hist();
    }
}

fn hist() {
    let stdin = io::stdin();
    let mut data: Vec<f64> = vec![];
    for line in stdin.lock().lines() {
        let line_text = match line {
            Ok(line) => line,
            Err(err) => panic!("IO error: {}", err),
        };
        data.push(line_text.parse::<f64>().unwrap());
    }
    let data = data;

    let max = data.iter().fold(-1./0., |a, &b| f64::max(a,b)) + 0.0000001;  // HORRIBLE!!! use a next_after equivalent
    let min = data.iter().fold(1./0., |a, &b| f64::min(a,b));

    let num_bins = 10;

    let bin_width = (max - min) / num_bins as f64;

    let mut bins = vec![0; num_bins];

    for &val in data.iter() {
        let bin = ((val - min) / bin_width) as usize;
        bins[bin] += 1;
    }

    println!("{:?}", bins);
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
