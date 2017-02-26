//! A module for plotting graphs

use std::collections::HashMap;
use std::iter::FromIterator;
use std;

use histogram;
use axis;

// Given a value like a tick label or a bin count,
// calculate how far from the x-axis it should be plotted
fn value_to_axis_cell_offset(value: f64, axis: &axis::Axis, face_cells: u32) -> u32 {
    let data_per_cell = (axis.max() - axis.min()) / face_cells as f64;
    ((value - axis.min()) / data_per_cell).round() as u32
}

/// Given a list of ticks to display,
/// the total scale of the axis
/// and the number of face cells to work with,
/// create a mapping of cell offset to tick value
fn tick_offset_map(axis: &axis::Axis, face_width: u32) -> HashMap<u32, f64> {
    axis.ticks()
        .iter()
        .map(|&tick| (value_to_axis_cell_offset(tick, axis, face_width), tick))
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

fn create_x_axis_labels(x_tick_map: &HashMap<u32, f64>) -> Vec<XAxisLabel> {
    let mut ls = Vec::from_iter(x_tick_map.iter().map(|(&offset, &tick)| {
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

    // Get the strings and offsets we'll use for the y-axis
    let largest_bin_count = *h.bin_counts.iter().max().expect("ERROR: There are no bins");
    let y_axis = axis::Axis::new(0.0, largest_bin_count as f64);
    let y_tick_map = tick_offset_map(&y_axis, face_height as u32);

    // Get the strings and offsets we'll use for the x-axis
    let x_min = *h.bin_bounds.first().expect("ERROR: There are no ticks for the x-axis");
    let x_max = *h.bin_bounds.last().expect("ERROR: There are no ticks for the x-axis");
    let x_axis = axis::Axis::new(x_min, x_max);
    let x_tick_map = tick_offset_map(&x_axis, face_width as u32);

    // Find a minimum size for the left gutter
    let longest_y_label_width = y_tick_map.values()
        .map(|n| n.to_string().len())
        .max()
        .expect("ERROR: There are no y-axis ticks");

    // Generate a list of strings to label the y-axis
    let y_label_strings = Vec::from_iter((0..face_height + 1)
        .map(|line| match y_tick_map.get(&line) {
            Some(v) => v.to_string(),
            None => "".to_string(),
        }));

    // Generate a list of strings to tick the y-axis
    let y_tick_strings = Vec::from_iter((0..face_height + 1)
        .map(|line| match y_tick_map.get(&line) {
            Some(_) => "-".to_string(),
            None => " ".to_string(),
        }));

    // Generate a list of strings to be the y-axis line itself
    let mut y_axis_line_strings = vec![];
    {
        let axis_corner = vec!["+".to_string()];
        let axis_lines = vec!["|".to_string(); face_height as usize];
        y_axis_line_strings.extend(axis_corner);
        y_axis_line_strings.extend(axis_lines);
    }
    let y_axis_line_strings = y_axis_line_strings;

    // Create a string which will be printed to give the x-axis tick marks
    let mut x_axis_tick_string = "".to_string();
    for cell in 0..face_width + 1 {
        let cell = cell as u32;
        let ch = if x_tick_map.get(&cell).is_some() {
            '|'
        } else {
            ' '
        };
        x_axis_tick_string.push(ch);
    }

    // Create a string which will be printed to give the x-axis labels
    let x_labels = create_x_axis_labels(&x_tick_map);
    let start_offset = x_labels.iter()
        .map(|label| label.start_offset())
        .min()
        .expect("ERROR: Could not compute start offset of x-axis");

    // This string will be printed, starting at start_offset relative to the x-axis zero cell
    let mut x_axis_label_string = "".to_string();
    for label in (&x_labels).iter() {
        let spaces_to_append = label.start_offset() - start_offset -
                               x_axis_label_string.len() as i32;
        if spaces_to_append.is_positive() {
            for _ in 0..spaces_to_append {
                x_axis_label_string.push(' ');
            }
        } else {
            for _ in 0..spaces_to_append.wrapping_neg() {
                x_axis_label_string.pop();
            }
        }
        let formatted_label = format!("{: ^footprint$}", label.text, footprint = label.footprint());
        x_axis_label_string.push_str(&formatted_label);
    }

    // Generate a list of strings to be the y-axis line itself
    let mut x_axis_line_string = String::new();
    {
        let axis_lines = vec!['-'; face_width as usize];
        x_axis_line_string.push('+');
        x_axis_line_string.extend(axis_lines);
    }
    let x_axis_line_string = x_axis_line_string;

    let mut face_strings: Vec<String> = vec![];

    for line in 0..face_height {
        let mut cols = String::new();
        for &bin_count in h.bin_counts.iter() {
            // between 0..1 how full the bin is compared to largest
            let bin_height_characters =
                value_to_axis_cell_offset(bin_count as f64, &y_axis, face_height);
            if bin_height_characters > line {
                cols.push_str("###");
            } else {
                cols.push_str("   ");
            }
        }
        face_strings.push(cols);
    }

    let left_gutter_width = std::cmp::max(longest_y_label_width as i32 + 1,
                                          (start_offset as i32).wrapping_neg()) as
                            usize;

    // Go, line-by-line printing the y-axis and the face
    for line in 0..face_height {
        let cell_position = face_height - line;
        let ref axis_label = y_label_strings[*&cell_position as usize];
        let ref axis_tick = y_tick_strings[*&cell_position as usize];
        let ref axis_line = y_axis_line_strings[*&cell_position as usize];
        let mut axis_label_and_tick = String::new();
        axis_label_and_tick.push_str(axis_label);
        axis_label_and_tick.push_str(axis_tick);
        let ref face_line = face_strings[(*&cell_position as usize) - 1];
        println!("{:>left_gutter_width$}{}{}",
                 axis_label_and_tick,
                 axis_line,
                 face_line,
                 left_gutter_width = left_gutter_width);
    }

    // Avoid duplicating the '+'. TODO combine them cleverly
    let reduced_x_axis_line = x_axis_line_string[1..].to_string();
    // Print the x-axis line and the zero-point of the y-axis
    println!("{:>left_gutter_width$}{}{}",
             y_label_strings[0],
             y_axis_line_strings[0],
             reduced_x_axis_line,
             left_gutter_width = left_gutter_width);

    // Print the x-axis ticks
    println!("{:>left_gutter_width$}{:-<plot_width$}",
             "",
             x_axis_tick_string,
             left_gutter_width = left_gutter_width,
             plot_width = face_width + 1);

    // Print the x-axis labels
    println!("{:>start_offset$}{: <plot_width$}",
             "",
             x_axis_label_string,
             start_offset = (left_gutter_width as i32 + start_offset) as usize,
             plot_width = (face_width as i32 + 1 + start_offset) as usize);
}

#[cfg(test)]
mod tests {
    use super::*;

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
