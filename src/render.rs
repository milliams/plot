//! A module for plotting graphs

use std::collections::HashMap;
use std::iter::FromIterator;

use histogram;

/// Given a list of ticks to display,
/// the total scale of the axis
/// and the number of lines to work with,
/// produce the label for each line of the axis
pub fn distribute_ticks_frequency(ticks: Vec<u32>, max: u32, lines: u32) -> Vec<String> {
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

/// Given a maximum frequency for the histogram, work out how many ticks to display on the y-axis
fn calc_tick_step_for_range(min: f64, max: f64) -> f64 {
    if max - min > 1.0 {
        // We are working on a larger range so we can use integer representations
        1.0
    } else {
        // We are on some small range so will need to be careful to represent the ticks correctly
        0.1 // TODO Be just a tad cleverer than this...
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
fn calculate_ticks_frequency(max: u32) -> Vec<u32> {
    let tick_step = calc_tick_step_for_frequency(max);
    Vec::from_iter((0..max + 1).filter(|i| i % tick_step == 0))
}

#[test]
fn test_calculate_tick_step() {
    use std::collections::HashMap;
    // For a given maximum count, check the step size
    let mut steps = HashMap::new();
    steps.insert(1..6, 1);
    steps.insert(6..12, 2);
    steps.insert(12..15, 4);
    steps.insert(15..30, 5);
    steps.insert(30..60, 10);
    steps.insert(60..120, 20);
    steps.insert(120..150, 40);
    steps.insert(150..300, 50);
    let steps = steps;

    for (range, step) in steps {
        for i in range {
            assert_eq!(calc_tick_step_for_frequency(i), step);
        }
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

pub fn draw_histogram(h: histogram::Histogram) {
    let plot_width = h.num_bins() * 3;
    let plot_height = 30;

    let largest_bin_count = h.bin_counts.iter().max().unwrap();

    let y_ticks = calculate_ticks_frequency(*largest_bin_count);

    let longest_y_label_width = y_ticks.iter().map(|n| n.to_string().len()).max().unwrap();

    let axis_strings = distribute_ticks_frequency(y_ticks.clone(),
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
}
