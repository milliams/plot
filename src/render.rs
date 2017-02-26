//! A module for plotting graphs

use std::collections::HashMap;
use std::iter::FromIterator;

use histogram;
use axis;

// Given a value like a tick label or a bin count,
// calculate how far from the x-axis it should be plotted
fn value_to_axis_cell_offset(value: f64, min: f64, max: f64, face_cells: u32) -> u32 {
    let data_per_cell = (max - min) / face_cells as f64;
    ((value - min) / data_per_cell).round() as u32
}

/// Given a list of ticks to display,
/// the total scale of the axis
/// and the number of lines to work with,
/// produce the label for each line of the axis
pub fn distribute_y_ticks(ticks: Vec<u32>, max: f64, face_lines: u32) -> Vec<String> {
    let m = tick_offset_map(&Vec::from_iter(ticks.iter().map(|&x| x as f64)),
                            0.0,
                            max,
                            face_lines);
    let p = (0..face_lines + 1).map(|line| if m.contains_key(&line) {
        m[&line].to_string()
    } else {
        "".to_string()
    });
    Vec::from_iter(p)
}

/// Given a list of ticks to display,
/// the total scale of the axis
/// and the number of face cells to work with,
/// create a mapping of cell offset to tick value
pub fn tick_offset_map(ticks: &Vec<f64>, min: f64, max: f64, face_width: u32) -> HashMap<u32, f64> {
    ticks.iter()
        .map(|&tick| (value_to_axis_cell_offset(tick, min, max, face_width), tick))
        .collect()
}

/// An x-axis label for the text output renderer
#[derive(Debug)]
struct XAxisLabel {
    text: String,
    offset: u32,
}

impl XAxisLabel {
    fn len(&self) -> usize {
        self.text.len()
    }

    /// The number of cells the label will actually use
    /// We want this to always be an odd number
    fn footprint(&self) -> usize {
        if self.len() % 2 == 0 {
            self.len() + 1
        } else {
            self.len()
        }
    }

    /// The offset, relative to the zero-point of the axis where the label should start to be drawn
    fn start_offset(&self) -> i32 {
        self.offset as i32 - self.footprint() as i32 / 2
    }
}

fn create_x_axis_labels(tick_map: &HashMap<u32, f64>) -> Vec<XAxisLabel> {
    let mut ls = Vec::from_iter(tick_map.iter().map(|(&offset, &tick)| {
        XAxisLabel {
            text: tick.to_string(),
            offset: offset,
        }
    }));
    ls.sort_by_key(|l| l.offset);
    ls
}

pub fn draw_histogram(h: histogram::Histogram) {
    // The face is the actual area of the graph with data on it, excluding axes and labels
    let face_width = h.num_bins() * 3;
    let face_height = 30;
    let max_y_ticks = 6;
    let max_x_ticks = 6;

    let largest_bin_count = *h.bin_counts.iter().max().expect("ERROR: There are no bins");

    let y_ticks = axis::calculate_ticks_frequency(largest_bin_count, max_y_ticks);

    let longest_y_label_width = y_ticks.iter()
        .map(|n| n.to_string().len())
        .max()
        .expect("ERROR: There are no y-axis ticks");

    let y_axis_strings = distribute_y_ticks(y_ticks, largest_bin_count as f64, face_height);

    let min = *h.bin_bounds.first().expect("ERROR: There are no ticks for the x-axis");
    let max = *h.bin_bounds.last().expect("ERROR: There are no ticks for the x-axis");
    let x_tick_step = axis::calculate_tick_step_for_range(min, max, max_x_ticks);
    let x_ticks = axis::generate_ticks(min, max, x_tick_step);
    let tick_map = tick_offset_map(&x_ticks, min, max, face_width as u32);
    let mut keys = Vec::from_iter(tick_map.keys());
    keys.sort();

    let mut tick_marks = "".to_string();
    for cell in 0..face_width + 1 {
        let cell = cell as u32;
        let ch = if tick_map.get(&cell).is_some() {
            '|'
        } else {
            ' '
        };
        tick_marks.push(ch);
    }

    let x_labels = create_x_axis_labels(&tick_map);
    let start_offset = x_labels.iter()
        .map(|label| label.start_offset())
        .min()
        .expect("ERROR: Could not compute start offset of x-axis");

    // This string will be printed, starting at start_offset relative to the x-axis zero cell
    let mut tick_label_string = "".to_string();

    for label in (&x_labels).iter() {
        let spaces_to_append = label.start_offset() - start_offset - tick_label_string.len() as i32;
        if spaces_to_append.is_positive() {
            for _ in 0..spaces_to_append {
                tick_label_string.push(' ');
            }
        } else {
            for _ in 0..spaces_to_append.wrapping_neg() {
                tick_label_string.pop();
            }
        }
        let formatted_label = format!("{: ^footprint$}", label.text, footprint = label.footprint());
        tick_label_string.push_str(&formatted_label);
    }

    for line in 0..face_height {
        let axis_label = y_axis_strings[(face_height - line) as usize].to_string();
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
             y_axis_strings[0],
             "",
             label_width = longest_y_label_width,
             plot_width = face_width);

    println!("{:>label_width$} {:-<plot_width$}",
             "",
             tick_marks,
             label_width = longest_y_label_width,
             plot_width = face_width + 1);

    println!("{:>label_width$} {: <plot_width$}",
             "",
             tick_label_string,
             label_width = (longest_y_label_width as i32 + start_offset) as usize,
             plot_width = (face_width as i32 + 1 + start_offset) as usize);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distribute_y_ticks() {
        assert_eq!(distribute_y_ticks(vec![0, 1, 2, 3, 4, 5], 5.0, 5),
                   ["0", "1", "2", "3", "4", "5"]);

        assert_eq!(distribute_y_ticks(vec![0, 1, 2, 3, 4, 5], 6.0, 7),
                   ["0", "1", "2", "", "3", "4", "5", ""]);
        assert_eq!(distribute_y_ticks(vec![0, 1, 2, 3, 4, 5], 6.0, 8),
                   ["0", "1", "", "2", "3", "4", "", "5", ""]);
        assert_eq!(distribute_y_ticks(vec![0, 1, 2, 3, 4, 5], 6.0, 9),
                   ["0", "", "1", "2", "", "3", "4", "", "5", ""]);
        assert_eq!(distribute_y_ticks(vec![0, 1, 2, 3, 4, 5], 6.0, 10),
                   ["0", "", "1", "2", "", "3", "", "4", "5", "", ""]);
    }

    #[test]
    fn test_x_axis_label() {
        let l = XAxisLabel {
            text: "3".to_string(),
            offset: 2,
        };
        assert_eq!(l.len(), 1);
        assert!(l.footprint() % 2 != 0);
        assert_eq!(l.start_offset(), 2);

        let l = XAxisLabel {
            text: "34".to_string(),
            offset: 2,
        };
        assert_eq!(l.len(), 2);
        assert!(l.footprint() % 2 != 0);
        assert_eq!(l.start_offset(), 1);

        let l = XAxisLabel {
            text: "345".to_string(),
            offset: 2,
        };
        assert_eq!(l.len(), 3);
        assert!(l.footprint() % 2 != 0);
        assert_eq!(l.start_offset(), 1);

        let l = XAxisLabel {
            text: "3454".to_string(),
            offset: 1,
        };
        assert_eq!(l.len(), 4);
        assert!(l.footprint() % 2 != 0);
        assert_eq!(l.start_offset(), -1);
    }
}
