//! A module for Histograms
//!
//! TODO:
//!  - frequency or density option
//!    - Variable bins implies frequency
//!    - What should be the default?

use std::iter::FromIterator;

#[derive(Debug)]
pub struct Histogram {
    pub bin_bounds: Vec<f64>, // will have N_bins + 1 entries
    pub bin_counts: Vec<i32>, // will have N_bins entries
    pub bin_densities: Vec<f64>, // will have N_bins entries
}

impl Histogram {
    pub fn from_vec(v: &Vec<f64>) -> Histogram {

        let max = v.iter().fold(-1. / 0., |a, &b| f64::max(a, b)) + 0.0000001; //TODO use next_after
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

/// Given a upper bound, calculate the sensible places to place the ticks
fn calculate_ticks_frequency(max: u32) {
    let num_ticks = 4;
}

#[test]
fn it_works() {}
