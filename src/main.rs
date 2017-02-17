extern crate clap;
use clap::{Arg, App, SubCommand};

use std::io::{self, BufRead};
use std::iter::FromIterator;

fn main() {
    let matches = App::new("Command-line stats")
        .about("Does awesome things")
        .subcommand(SubCommand::with_name("average")
            .about("computes the average of the input stream"))
        .subcommand(SubCommand::with_name("hist").about("plots a histogram of the data"))
        .get_matches();

    if matches.subcommand_matches("average").is_some() {
        average();
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

#[derive(Debug)]
struct Histogram {
    bin_bounds: Vec<f64>, // will have N_bins + 1 entries
    bin_counts: Vec<i32>, // will have N_bins entries
    bin_densities: Vec<f64>, // will have N_bins entries
}

impl Histogram {
    pub fn from_vec(v: &Vec<f64>) -> Histogram {

        let max = v.iter().fold(-1. / 0., |a, &b| f64::max(a, b)) + 0.0000001; // TODO: use a next_after equivalent
        let min = v.iter().fold(1. / 0., |a, &b| f64::min(a, b));

        let num_bins = 10; // Number of bins

        let mut bins = vec![0; num_bins];

        let bin_width = (max - min) / num_bins as f64; // width of bin in real units

        for &val in v.iter() {
            let bin = ((val - min) / bin_width) as usize;
            bins[bin] += 1;
        }
        let density_per_bin = Vec::from_iter(bins.iter().map(|&x| x as f64 / bin_width));

        Histogram {
            bin_bounds: vec![], // TODO
            bin_counts: bins,
            bin_densities: density_per_bin,
        }
    }

    pub fn num_bins(&self) -> usize {
        self.bin_counts.len()
    }
}

fn hist() {
    let data = get_single_column();

    let h = Histogram::from_vec(&data);

    let plot_width = h.num_bins() * 3;
    let plot_height = 30;

    let largest_bin_count = h.bin_counts.iter().max().unwrap();

    let longest_y_label_width = largest_bin_count.to_string().len();

    for line in 0..plot_height {
        let axis_label = " ".to_string(); // TODO: or largest_bin_count or blank
        let mut cols = String::new();
        for &bin_count in h.bin_counts.iter() {
            let bin_height_fraction = bin_count as f32 / *largest_bin_count as f32; // between 0..1 how full the bin is compared to largest
            let bin_height_characters = (bin_height_fraction * plot_height as f32) as i32;
            if bin_height_characters >= plot_height - line {
                cols.push_str("###");
            } else {
                cols.push_str("   ");
            }
        }

        println!("{:>label_width$} |{}",
                 axis_label,
                 cols,
                 label_width = longest_y_label_width);
    }
    println!("{:>label_width$} +{:-<plot_width$}",
             " ",
             "",
             label_width = longest_y_label_width,
             plot_width = plot_width);

    //println!("{:?}", bins);
}

/// Given a range of numbers, calculate where is best to place the ticks
fn calculate_ticks(lower: f64, upper: f64) {}

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

#[test]
fn it_works() {}
