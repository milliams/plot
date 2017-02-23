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
    pub bin_counts: Vec<u32>, // will have N_bins entries
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
pub fn calculate_ticks_frequency(max: u32) -> Vec<u32> {
    let tick_step = calc_tick_step_for_frequency(max);
    Vec::from_iter((0..max + 1).filter(|i| i % tick_step == 0))
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

#[test]
fn test_calculate_ticks() {
    assert_eq!(calculate_ticks_frequency(1), [0, 1]); // step up in 1s
    assert_eq!(calculate_ticks_frequency(2), [0, 1, 2]);
    assert_eq!(calculate_ticks_frequency(3), [0, 1, 2, 3]); // step up in 1s
    assert_eq!(calculate_ticks_frequency(4), [0, 1, 2, 3, 4]);
    assert_eq!(calculate_ticks_frequency(5), [0, 1, 2, 3, 4, 5]);
    assert_eq!(calculate_ticks_frequency(6), [0, 2, 4, 6]); // step up in 2s
    assert_eq!(calculate_ticks_frequency(7), [0, 2, 4, 6]);
    assert_eq!(calculate_ticks_frequency(8), [0, 2, 4, 6, 8]);
    assert_eq!(calculate_ticks_frequency(9), [0, 2, 4, 6, 8]);
    assert_eq!(calculate_ticks_frequency(10), [0, 2, 4, 6, 8, 10]);
    assert_eq!(calculate_ticks_frequency(11), [0, 2, 4, 6, 8, 10]);
    assert_eq!(calculate_ticks_frequency(12), [0, 4, 8, 12]); // step up in 4s
    assert_eq!(calculate_ticks_frequency(13), [0, 4, 8, 12]);
    assert_eq!(calculate_ticks_frequency(14), [0, 4, 8, 12]);
    assert_eq!(calculate_ticks_frequency(15), [0, 5, 10, 15]); // step up in 5s
    assert_eq!(calculate_ticks_frequency(16), [0, 5, 10, 15]);
    assert_eq!(calculate_ticks_frequency(17), [0, 5, 10, 15]);
    assert_eq!(calculate_ticks_frequency(18), [0, 5, 10, 15]);
    assert_eq!(calculate_ticks_frequency(19), [0, 5, 10, 15]);
    assert_eq!(calculate_ticks_frequency(20), [0, 5, 10, 15, 20]);
    assert_eq!(calculate_ticks_frequency(21), [0, 5, 10, 15, 20]);
    assert_eq!(calculate_ticks_frequency(22), [0, 5, 10, 15, 20]);
    assert_eq!(calculate_ticks_frequency(23), [0, 5, 10, 15, 20]);
    assert_eq!(calculate_ticks_frequency(24), [0, 5, 10, 15, 20]);
    assert_eq!(calculate_ticks_frequency(25), [0, 5, 10, 15, 20, 25]);
    assert_eq!(calculate_ticks_frequency(26), [0, 5, 10, 15, 20, 25]);
    assert_eq!(calculate_ticks_frequency(27), [0, 5, 10, 15, 20, 25]);
    assert_eq!(calculate_ticks_frequency(28), [0, 5, 10, 15, 20, 25]);
    assert_eq!(calculate_ticks_frequency(29), [0, 5, 10, 15, 20, 25]);
    assert_eq!(calculate_ticks_frequency(30), [0, 10, 20, 30]); // step up in 10s
    assert_eq!(calculate_ticks_frequency(31), [0, 10, 20, 30]);
    //...
    assert_eq!(calculate_ticks_frequency(40), [0, 10, 20, 30, 40]);
    assert_eq!(calculate_ticks_frequency(50), [0, 10, 20, 30, 40, 50]);
    assert_eq!(calculate_ticks_frequency(60), [0, 20, 40, 60]); // step up in 20s
    assert_eq!(calculate_ticks_frequency(70), [0, 20, 40, 60]);
    assert_eq!(calculate_ticks_frequency(80), [0, 20, 40, 60, 80]);
    assert_eq!(calculate_ticks_frequency(90), [0, 20, 40, 60, 80]);
    assert_eq!(calculate_ticks_frequency(100), [0, 20, 40, 60, 80, 100]);
    assert_eq!(calculate_ticks_frequency(110), [0, 20, 40, 60, 80, 100]);
    assert_eq!(calculate_ticks_frequency(120), [0, 40, 80, 120]); // step up in 40s
    assert_eq!(calculate_ticks_frequency(130), [0, 40, 80, 120]);
    assert_eq!(calculate_ticks_frequency(140), [0, 40, 80, 120]);
    assert_eq!(calculate_ticks_frequency(150), [0, 50, 100, 150]); // step up in 50s
    //...
    assert_eq!(calculate_ticks_frequency(3475), [0, 1000, 2000, 3000]);
}
