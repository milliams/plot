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


/// A slow way to round an integer down to a number of significant figures
fn round_down_to_sig_fig(value: u32, figures: usize) -> u32 {
    let s = value.to_string();
    let r_map = s.chars().enumerate().map(|(i, ch)| if i >= figures { '0' } else { ch });
    let r = String::from_iter(r_map);
    r.parse::<u32>().unwrap()
}

/// Given a maximum frequency for the histogram, work out how many ticks to display on the y-axis
fn calc_tick_step_for_frequency(max: u32) -> u32 {
    let base_steps = [1, 2, 4, 5]; // sensible types of step values
    // We want to scale the base_steps by some power of 10
    let base_step_scale = ((max / 3) as f64).log10() as u32;
    let steps = Vec::from_iter(base_steps.iter().map(|s| s * 10u32.pow(base_step_scale)));

    let default_step = 1;
    *steps.iter().rev().find(|&try_step| max / try_step >= 3).unwrap_or(&default_step)
}

/// Given a upper bound, calculate the sensible places to place the ticks
fn calculate_ticks_frequency(max: u32) {
    let top_value = round_down_to_sig_fig(max, 1);
}

#[test]
fn test_rounding() {
    assert_eq!(round_down_to_sig_fig(10, 1), 10);
    assert_eq!(round_down_to_sig_fig(11, 1), 10);
    assert_eq!(round_down_to_sig_fig(4573, 2), 4500);
}

#[test]
fn test_calculate_tick_step() {
    for i in 1..6 {
        assert_eq!(calc_tick_step_for_frequency(i), 1);
    }
    for i in 6..12 {
        assert_eq!(calc_tick_step_for_frequency(i), 2);
    }
    for i in 12..15 {
        assert_eq!(calc_tick_step_for_frequency(i), 4);
    }
    for i in 15..30 {
        assert_eq!(calc_tick_step_for_frequency(i), 5);
    }
    for i in 30..60 {
        assert_eq!(calc_tick_step_for_frequency(i), 10);
    }
    for i in 60..120 {
        assert_eq!(calc_tick_step_for_frequency(i), 20);
    }
    for i in 120..150 {
        assert_eq!(calc_tick_step_for_frequency(i), 40);
    }
    for i in 150..300 {
        assert_eq!(calc_tick_step_for_frequency(i), 50);
    }
}
