//! A module for plotting graphs

use std::collections::HashMap;
use std::iter::FromIterator;

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
