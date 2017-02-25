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

/// The base units for the step sizes
/// They should be within one order of magnitude, e.g. [1,10)
const BASE_STEPS: [u32; 4] = [1, 2, 4, 5];

#[derive(Debug)]
struct TickSteps {
    next: u32,
}

impl TickSteps {
    fn new() -> TickSteps {
        TickSteps::start_at(1)
    }

    fn start_at(start: u32) -> TickSteps {
        let start_options = TickSteps::scaled_steps(&start);
        let overflow: u32 = start_options[0] * 10;
        let curr = start_options.iter().skip_while(|&step| step < &start).next();

        TickSteps { next: *curr.unwrap_or(&overflow) }
    }

    fn scaled_steps(curr: &u32) -> Vec<u32> {
        let base_step_scale = 10u32.pow((*curr as f64).log10() as u32);
        Vec::from_iter(BASE_STEPS.iter().map(|s| s * base_step_scale))
    }
}

impl Iterator for TickSteps {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let curr = self.next; // cache the value we're currently on
        let curr_steps = TickSteps::scaled_steps(&self.next);
        let overflow: u32 = curr_steps[0] * 10;
        self.next = *curr_steps.iter().skip_while(|&s| s <= &curr).next().unwrap_or(&overflow);
        Some(curr)
    }
}

#[test]
fn test_tick_step_generator() {
    let t = TickSteps::new();
    let ts = Vec::from_iter(t.take(7));
    assert_eq!(ts, [1, 2, 4, 5, 10, 20, 40]);

    let t = TickSteps::start_at(100);
    let ts = Vec::from_iter(t.take(5));
    assert_eq!(ts, [100, 200, 400, 500, 1000]);

    let t = TickSteps::start_at(3);
    let ts = Vec::from_iter(t.take(5));
    assert_eq!(ts, [4, 5, 10, 20, 40]);

    let t = TickSteps::start_at(3.6 as u32);
    let ts = Vec::from_iter(t.take(3));
    assert_eq!(ts, [4, 5, 10]);

    let t = TickSteps::start_at(8);
    let ts = Vec::from_iter(t.take(3));
    assert_eq!(ts, [10, 20, 40]);

    let t = TickSteps::start_at(7.5 as u32);
    let ts = Vec::from_iter(t.take(3));
    assert_eq!(ts, [10, 20, 40]);
}

/// Given a range of values, and a maximum number fo ticks, calulate the step between the ticks
fn calculate_tick_step_for_range(min: f64, max: f64, max_ticks: i32) -> f64 {
    let range = max - min;
    if range > 1.0 {
        let min_tick_step = range / max_ticks as f64;
        let tick_step = TickSteps::start_at(min_tick_step as u32).next();
        tick_step.expect("ERROR: We've somehow run out of tick step options!") as f64
    } else {
        // We are on some small range so will need to be careful to represent the ticks correctly
        0.1 // TODO Be just a tad cleverer than this...
    }
}

#[test]
fn test_calculate_tick_step_for_range() {
    assert_eq!(calculate_tick_step_for_range(-7.93, 15.58, 6), 4.0);
}

/// Given a maximum frequency for the histogram, work out how many ticks to display on the y-axis
fn calculate_tick_step_for_frequency(max: u32) -> u32 {
    // We want to scale the BASE_STEPS by some power of 10
    let base_step_scale = ((max / 3) as f64).log10() as u32;
    let steps = Vec::from_iter(BASE_STEPS.iter().map(|s| s * 10u32.pow(base_step_scale)));

    let default_step = 1;
    *steps.iter().rev().find(|&try_step| max / try_step >= 3).unwrap_or(&default_step)
}

/// Given a upper bound, calculate the sensible places to place the ticks
fn calculate_ticks_frequency(max: u32) -> Vec<u32> {
    let tick_step = calculate_tick_step_for_frequency(max);
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
            assert_eq!(calculate_tick_step_for_frequency(i), step);
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
