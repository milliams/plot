//! A module for plotting graphs

use std::collections::HashMap;
use std::iter::FromIterator;

use histogram;

/// The base units for the step sizes
/// They should be within one order of magnitude, e.g. [1,10)
const BASE_STEPS: [u32; 4] = [1, 2, 4, 5];

#[derive(Debug,Clone)]
struct TickSteps {
    next: u32,
}

impl TickSteps {
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
    let t = TickSteps::start_at(1);
    let ts = Vec::from_iter(t.take(7));
    assert_eq!(ts, [1, 2, 4, 5, 10, 20, 40]);

    let t = TickSteps::start_at(100);
    let ts = Vec::from_iter(t.take(5));
    assert_eq!(ts, [100, 200, 400, 500, 1000]);

    let t = TickSteps::start_at(3);
    let ts = Vec::from_iter(t.take(5));
    assert_eq!(ts, [4, 5, 10, 20, 40]);

    let t = TickSteps::start_at(8);
    let ts = Vec::from_iter(t.take(3));
    assert_eq!(ts, [10, 20, 40]);
}

fn generate_ticks(min: f64, max: f64, step_size: f64) -> Vec<f64> {
    let mut ticks: Vec<f64> = vec![];
    if min <= 0.0 {
        if max >= 0.0 {
            // standard spanning axis
            ticks.push(0.0);
            ticks.extend((1..).map(|n| -1.0 * n as f64 * step_size).take_while(|&v| v >= min));
            ticks.extend((1..).map(|n| n as f64 * step_size).take_while(|&v| v <= max));
        } else {
            // entirely negative axis
            ticks.extend((1..)
                .map(|n| -1.0 * n as f64 * step_size)
                .skip_while(|&v| v > max)
                .take_while(|&v| v >= min));
        }
    } else {
        // entirely positive axis
        ticks.extend((1..)
            .map(|n| n as f64 * step_size)
            .skip_while(|&v| v < min)
            .take_while(|&v| v <= max));
    }
    ticks.sort_by(|a, b| a.partial_cmp(b).unwrap());
    ticks
}

/// Given a range and a step size, work out how many ticks will be displayed
fn number_of_ticks(min: f64, max: f64, step_size: f64) -> u32 {
    generate_ticks(min, max, step_size).len() as u32
}

#[test]
fn test_number_of_ticks() {
    assert_eq!(number_of_ticks(-7.93, 15.58, 4.0), 5);
    assert_eq!(number_of_ticks(-7.93, 15.58, 5.0), 5);
    assert_eq!(number_of_ticks(0.0, 15.0, 4.0), 4);
    assert_eq!(number_of_ticks(0.0, 15.0, 5.0), 4);
    assert_eq!(number_of_ticks(5.0, 21.0, 4.0), 4);
    assert_eq!(number_of_ticks(5.0, 21.0, 5.0), 4);
    assert_eq!(number_of_ticks(-8.0, 15.58, 4.0), 6);
    assert_eq!(number_of_ticks(-8.0, 15.58, 5.0), 5);
}

/// Given a range of values, and a maximum number of ticks, calulate the step between the ticks
fn calculate_tick_step_for_range(min: f64, max: f64, max_ticks: i32) -> f64 {
    let range = max - min;
    let min_tick_step = ((range / max_ticks as f64) + 1.0) as u32;
    if range > 1.0 {
        // Get our generator of tick step sizes
        let tick_steps = TickSteps::start_at(min_tick_step);
        // Get the first entry which is our smallest possible tick step size
        let smallest_valid_step = tick_steps.clone()
            .next()
            .expect("ERROR: We've somehow run out of tick step options!") as
                                  f64;
        // Count how many ticks that relates to
        let actual_num_ticks = number_of_ticks(min, max, smallest_valid_step);
        // Get all the possible tick step sizes that give just as many ticks
        let step_options =
            tick_steps.take_while(|&s| number_of_ticks(min, max, s as f64) == actual_num_ticks);
        // Get the largest tick step size from the list
        step_options.max().expect("ERROR: No tick options") as f64
    } else {
        // We are on some small range so will need to be careful to represent the ticks correctly
        0.1 // TODO Be just a tad cleverer than this...
    }
}

#[test]
fn test_calculate_tick_step_for_range() {
    assert_eq!(calculate_tick_step_for_range(0.0, 6.0, 6), 2.0);
    assert_eq!(calculate_tick_step_for_range(0.0, 11.0, 6), 2.0);
    assert_eq!(calculate_tick_step_for_range(0.0, 14.0, 6), 4.0);
    assert_eq!(calculate_tick_step_for_range(0.0, 15.0, 6), 5.0);
    assert_eq!(calculate_tick_step_for_range(-1.0, 5.0, 6), 2.0);
    assert_eq!(calculate_tick_step_for_range(-7.93, 15.58, 6), 5.0);
}

/// Given a maximum frequency for the histogram and a maximum number of ticks,
/// work out the step between ticks
fn calculate_tick_step_for_frequency(max: u32, max_ticks: i32) -> u32 {
    calculate_tick_step_for_range(0.0, max as f64, max_ticks) as u32
}

/// Given a upper bound, calculate the sensible places to place the ticks
fn calculate_ticks_frequency(max: u32, max_ticks: i32) -> Vec<u32> {
    let tick_step = calculate_tick_step_for_frequency(max, max_ticks);
    Vec::from_iter(generate_ticks(0.0, max as f64, tick_step as f64).iter().map(|&t| t as u32))
}

#[test]
fn test_calculate_ticks() {
    //assert_eq!(calculate_ticks_frequency(1), [0, 1]); // step up in 1s
    assert_eq!(calculate_ticks_frequency(2, 6), [0, 1, 2]);
    assert_eq!(calculate_ticks_frequency(3, 6), [0, 1, 2, 3]);
    assert_eq!(calculate_ticks_frequency(4, 6), [0, 1, 2, 3, 4]);
    assert_eq!(calculate_ticks_frequency(5, 6), [0, 1, 2, 3, 4, 5]);
    assert_eq!(calculate_ticks_frequency(6, 6), [0, 2, 4, 6]);
    assert_eq!(calculate_ticks_frequency(7, 6), [0, 2, 4, 6]);
    assert_eq!(calculate_ticks_frequency(8, 6), [0, 2, 4, 6, 8]);
    assert_eq!(calculate_ticks_frequency(9, 6), [0, 2, 4, 6, 8]);
    assert_eq!(calculate_ticks_frequency(10, 6), [0, 2, 4, 6, 8, 10]);
    assert_eq!(calculate_ticks_frequency(11, 6), [0, 2, 4, 6, 8, 10]);
    assert_eq!(calculate_ticks_frequency(12, 6), [0, 4, 8, 12]);
    assert_eq!(calculate_ticks_frequency(13, 6), [0, 4, 8, 12]);
    assert_eq!(calculate_ticks_frequency(14, 6), [0, 4, 8, 12]);
    assert_eq!(calculate_ticks_frequency(15, 6), [0, 5, 10, 15]);
    assert_eq!(calculate_ticks_frequency(16, 6), [0, 4, 8, 12, 16]);
    assert_eq!(calculate_ticks_frequency(17, 6), [0, 4, 8, 12, 16]);
    assert_eq!(calculate_ticks_frequency(18, 6), [0, 4, 8, 12, 16]);
    assert_eq!(calculate_ticks_frequency(19, 6), [0, 4, 8, 12, 16]);
    assert_eq!(calculate_ticks_frequency(20, 6), [0, 4, 8, 12, 16, 20]);
    assert_eq!(calculate_ticks_frequency(21, 6), [0, 4, 8, 12, 16, 20]);
    assert_eq!(calculate_ticks_frequency(22, 6), [0, 4, 8, 12, 16, 20]);
    assert_eq!(calculate_ticks_frequency(23, 6), [0, 4, 8, 12, 16, 20]);
    assert_eq!(calculate_ticks_frequency(24, 6), [0, 5, 10, 15, 20]);
    assert_eq!(calculate_ticks_frequency(25, 6), [0, 5, 10, 15, 20, 25]);
    assert_eq!(calculate_ticks_frequency(26, 6), [0, 5, 10, 15, 20, 25]);
    assert_eq!(calculate_ticks_frequency(27, 6), [0, 5, 10, 15, 20, 25]);
    assert_eq!(calculate_ticks_frequency(28, 6), [0, 5, 10, 15, 20, 25]);
    assert_eq!(calculate_ticks_frequency(29, 6), [0, 5, 10, 15, 20, 25]);
    assert_eq!(calculate_ticks_frequency(30, 6), [0, 10, 20, 30]);
    assert_eq!(calculate_ticks_frequency(31, 6), [0, 10, 20, 30]);
    //...
    assert_eq!(calculate_ticks_frequency(40, 6), [0, 10, 20, 30, 40]);
    assert_eq!(calculate_ticks_frequency(50, 6), [0, 10, 20, 30, 40, 50]);
    assert_eq!(calculate_ticks_frequency(60, 6), [0, 20, 40, 60]);
    assert_eq!(calculate_ticks_frequency(70, 6), [0, 20, 40, 60]);
    assert_eq!(calculate_ticks_frequency(80, 6), [0, 20, 40, 60, 80]);
    assert_eq!(calculate_ticks_frequency(90, 6), [0, 20, 40, 60, 80]);
    assert_eq!(calculate_ticks_frequency(100, 6), [0, 20, 40, 60, 80, 100]);
    assert_eq!(calculate_ticks_frequency(110, 6), [0, 20, 40, 60, 80, 100]);
    assert_eq!(calculate_ticks_frequency(120, 6), [0, 40, 80, 120]);
    assert_eq!(calculate_ticks_frequency(130, 6), [0, 40, 80, 120]);
    assert_eq!(calculate_ticks_frequency(140, 6), [0, 40, 80, 120]);
    assert_eq!(calculate_ticks_frequency(150, 6), [0, 50, 100, 150]);
    //...
    assert_eq!(calculate_ticks_frequency(3475, 6), [0, 1000, 2000, 3000]);
}

// Given a value like a tick label or a bin count,
// calculate how far from the x-axis it should be plotted
fn value_to_distance_from_x_axis(value: u32, max: u32, face_lines: u32) -> u32 {
    let data_per_cell_height = max as f64 / face_lines as f64;
    (value as f64 / data_per_cell_height).round() as u32
}

/// Given a list of ticks to display,
/// the total scale of the axis
/// and the number of lines to work with,
/// produce the label for each line of the axis
pub fn distribute_y_ticks(ticks: Vec<u32>, max: u32, face_lines: u32) -> Vec<String> {
    // map of distance from x-axis to tick value
    let m: HashMap<_, _> = ticks.iter()
        .map(|&tick| (value_to_distance_from_x_axis(tick, max, face_lines), tick))
        .collect();
    let p = (0..face_lines + 1).map(|line| if m.contains_key(&line) {
        m[&line].to_string()
    } else {
        "".to_string()
    });
    Vec::from_iter(p)
}

#[test]
fn test_distribute_ticks() {
    assert_eq!(distribute_y_ticks(vec![0, 1, 2, 3, 4, 5], 5, 5),
               ["0", "1", "2", "3", "4", "5"]);

    assert_eq!(distribute_y_ticks(vec![0, 1, 2, 3, 4, 5], 6, 7),
               ["0", "1", "2", "", "3", "4", "5", ""]);
    assert_eq!(distribute_y_ticks(vec![0, 1, 2, 3, 4, 5], 6, 8),
               ["0", "1", "", "2", "3", "4", "", "5", ""]);
    assert_eq!(distribute_y_ticks(vec![0, 1, 2, 3, 4, 5], 6, 9),
               ["0", "", "1", "2", "", "3", "4", "", "5", ""]);
    assert_eq!(distribute_y_ticks(vec![0, 1, 2, 3, 4, 5], 6, 10),
               ["0", "", "1", "2", "", "3", "", "4", "5", "", ""]);
}

pub fn draw_histogram(h: histogram::Histogram) {
    // The face is the actual area of the graph with data on it, excluding axes and labels
    let face_width = h.num_bins() * 3;
    let face_height = 30;
    let max_y_ticks = 6;

    let largest_bin_count = *h.bin_counts.iter().max().unwrap();

    let y_ticks = calculate_ticks_frequency(largest_bin_count, max_y_ticks);

    let longest_y_label_width = y_ticks.iter().map(|n| n.to_string().len()).max().unwrap();

    let axis_strings = distribute_y_ticks(y_ticks, largest_bin_count, face_height);

    for line in 0..face_height {
        let axis_label = axis_strings[(face_height - line) as usize].to_string();
        let mut cols = String::new();
        for &bin_count in h.bin_counts.iter() {
            // between 0..1 how full the bin is compared to largest
            let bin_height_fraction = bin_count as f32 / largest_bin_count as f32;
            let bin_height_characters = (bin_height_fraction * face_height as f32) as u32;
            if bin_height_characters == face_height - line {
                cols.push_str("---");
            } else if bin_height_characters > face_height - line {
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
             axis_strings[0],
             "",
             label_width = longest_y_label_width,
             plot_width = face_width);
}
