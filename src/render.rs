//! A module for plotting graphs

use std::collections::HashMap;
use std::iter::FromIterator;

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

    let largest_bin_count = *h.bin_counts.iter().max().expect("ERROR: There are no bins");

    let y_axis = axis::Axis::new(0.0, largest_bin_count as f64);
    let y_ticks = y_axis.ticks();
    let y_tick_map = tick_offset_map(&y_axis, face_height as u32);

    let longest_y_label_width = y_ticks.iter()
        .map(|n| n.to_string().len())
        .max()
        .expect("ERROR: There are no y-axis ticks");

    let min = *h.bin_bounds.first().expect("ERROR: There are no ticks for the x-axis");
    let max = *h.bin_bounds.last().expect("ERROR: There are no ticks for the x-axis");
    let x_axis = axis::Axis::new(min, max);
    let x_tick_map = tick_offset_map(&x_axis, face_width as u32);

    let mut tick_marks = "".to_string();
    for cell in 0..face_width + 1 {
        let cell = cell as u32;
        let ch = if x_tick_map.get(&cell).is_some() {
            '|'
        } else {
            ' '
        };
        tick_marks.push(ch);
    }

    let x_labels = create_x_axis_labels(&x_tick_map);
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
        let axis_label = match y_tick_map.get(&(face_height - line)) {
            Some(v) => v.to_string(),
            None => "".to_string(),
        };
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
             "0",
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
