//! A command-line plotting tool

extern crate plot;

extern crate clap;
use clap::{Arg, App, SubCommand};

fn main() {
    let matches = App::new("Command-line stats")
        .about("Does awesome things")
        .subcommand(SubCommand::with_name("average")
            .about("computes the average of the input stream"))
        .subcommand(SubCommand::with_name("hist").about("plots a histogram of the data"))
        .subcommand(SubCommand::with_name("stats").about("print some statistica about the data"))
        .get_matches();

    if matches.subcommand_matches("average").is_some() {
        plot::average();
    }

    if matches.subcommand_matches("stats").is_some() {
        plot::stats();
    }

    if matches.subcommand_matches("hist").is_some() {
        plot::hist();
    }
}
