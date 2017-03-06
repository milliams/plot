//! A command-line plotting tool

extern crate plot;

extern crate clap;
use clap::{Arg, App, SubCommand};

use plot::HistogramConfig;

fn main() {
    let hist_s = SubCommand::with_name("hist")
        .about("plots a histogram of the data")
        .arg(Arg::with_name("nbins")
            .long("nbins")
            .help("number of bins")
            .takes_value(true)
            .default_value("30"));

    let scatter_s = SubCommand::with_name("scatter")
        .about("plots a scatter plot of the data");

    let matches = App::new("plot")
        .about("Does awesome things")
        .subcommand(SubCommand::with_name("average")
            .about("computes the average of the input stream"))
        .subcommand(hist_s)
        .subcommand(scatter_s)
        .subcommand(SubCommand::with_name("stats").about("print some statistics about the data"))
        .get_matches();

    match matches.subcommand() {
        ("average", Some(_)) => {
            plot::average();
        }
        ("stats", Some(_)) => {
            plot::stats();
        }
        ("scatter", Some(_)) => {
            plot::scatter();
        }
        ("hist", Some(sub)) => {
            let nbins = sub.value_of("nbins").unwrap().parse().unwrap();

            let config = HistogramConfig { nbins: nbins };

            plot::hist(config);
        }
        _ => {} // some error
    }
}
