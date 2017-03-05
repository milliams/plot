//! A command-line plotting tool

extern crate plot;

extern crate clap;
use clap::{Arg, App, SubCommand};

use plot::HistogramConfig;

fn main() {
    let matches = App::new("plot")
        .about("Does awesome things")
        .subcommand(SubCommand::with_name("average")
            .about("computes the average of the input stream"))
        .subcommand(SubCommand::with_name("hist")
            .about("plots a histogram of the data")
            .arg(Arg::with_name("nbins")
                .long("nbins")
                .help("number of bins")
                .takes_value(true)
                .default_value("30")))
        .subcommand(SubCommand::with_name("stats").about("print some statistics about the data"))
        .get_matches();

    if matches.subcommand_matches("average").is_some() {
        plot::average();
    }

    if matches.subcommand_matches("stats").is_some() {
        plot::stats();
    }

    if matches.subcommand_matches("hist").is_some() {
        let subcommand = matches.subcommand_matches("hist").unwrap();
        let nbins = subcommand.value_of("nbins").unwrap();
        let nbins = nbins.parse().unwrap();

        let config = HistogramConfig {nbins: nbins};

        plot::hist(config);
    }
}
