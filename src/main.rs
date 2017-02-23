//! A command-line plotting tool

extern crate clap;
use clap::{Arg, App, SubCommand};

use std::io::{self, BufRead};
use std::iter::FromIterator;
use std::collections::HashMap;

mod histogram;

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

/// Given a list of ticks to display,
/// the total scale of the axis
/// and the number of lines to work with,
/// produce the label for each line of the axis
fn distribute_ticks_frequency(ticks: Vec<u32>, max: u32, lines: u32) -> Vec<String> {
    let m: HashMap<_, _> = ticks.iter()
        .map(|&tick| (((tick as f64 / max as f64) * lines as f64) as u32, tick))
        .collect();
    let p = (0..lines).map(|line| if m.contains_key(&line) {
        m[&line].to_string()
    } else {
        "".to_string()
    });
    Vec::from_iter(p)
}

#[test]
fn test_distribute_ticks() {
    assert_eq!(distribute_ticks_frequency(vec![0, 1, 2, 3, 4, 5], 6, 7),
               ["0", "1", "2", "3", "4", "5", ""]);
    assert_eq!(distribute_ticks_frequency(vec![0, 1, 2, 3, 4, 5], 6, 8),
               ["0", "1", "2", "", "3", "4", "5", ""]);
    assert_eq!(distribute_ticks_frequency(vec![0, 1, 2, 3, 4, 5], 6, 9),
               ["0", "1", "", "2", "3", "", "4", "5", ""]);
    assert_eq!(distribute_ticks_frequency(vec![0, 1, 2, 3, 4, 5], 6, 10),
               ["0", "1", "", "2", "", "3", "4", "", "5", ""]);
}

fn hist() {
    let data = get_single_column();

    let h = histogram::Histogram::from_vec(&data);

    let plot_width = h.num_bins() * 3;
    let plot_height = 30;

    let largest_bin_count = h.bin_counts.iter().max().unwrap();

    let ticks = histogram::calculate_ticks_frequency(*largest_bin_count);

    let longest_y_label_width = ticks.iter().map(|n| n.to_string().len()).max().unwrap();

    let axis_strings = distribute_ticks_frequency(ticks.clone(),
                                                  *h.bin_counts.iter().max().unwrap(),
                                                  plot_height);

    for line in 0..plot_height {
        let axis_label = axis_strings[(plot_height - line - 1) as usize].to_string();
        let mut cols = String::new();
        for &bin_count in h.bin_counts.iter() {
            // between 0..1 how full the bin is compared to largest
            let bin_height_fraction = bin_count as f32 / *largest_bin_count as f32;
            let bin_height_characters = (bin_height_fraction * plot_height as f32) as u32;
            if bin_height_characters == plot_height - line {
                cols.push_str("___");
            } else if bin_height_characters > plot_height - line {
                cols.push_str("| |");
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
