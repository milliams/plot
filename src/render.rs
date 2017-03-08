//! A module for plotting graphs

use std::collections::HashMap;
use std;

use plotlib::histogram;
use plotlib::scatter;
use plotlib::axis;
use plotlib::utils::PairWise;

// Given a value like a tick label or a bin count,
// calculate how far from the x-axis it should be plotted
fn value_to_axis_cell_offset(value: f64, axis: &axis::Axis, face_cells: u32) -> i32 {
    let data_per_cell = (axis.max() - axis.min()) / face_cells as f64;
    ((value - axis.min()) / data_per_cell).round() as i32
}

/// Given a list of ticks to display,
/// the total scale of the axis
/// and the number of face cells to work with,
/// create a mapping of cell offset to tick value
fn tick_offset_map(axis: &axis::Axis, face_width: u32) -> HashMap<i32, f64> {
    axis.ticks()
        .iter()
        .map(|&tick| (value_to_axis_cell_offset(tick, axis, face_width), tick))
        .collect()
}

/// Given a histogram object,
/// the total scale of the axis
/// and the number of face cells to work with,
/// create a mapping of cell offset to bin bound
fn bound_cell_offsets(hist: &histogram::Histogram, face_width: u32) -> Vec<i32> {
    hist.bin_bounds
        .iter()
        .map(|&bound| value_to_axis_cell_offset(bound, &hist.x_axis, face_width))
        .collect()
}

/// calculate for each cell which bin it is representing
/// Cells which straddle bins will return the bin just on the lower side of the centre of the cell
/// Will return a vector with (`face_width + 2`) entries to represent underflow and overflow cells
/// cells which do not map to a bin will return `None`.
fn bins_for_cells(bound_cell_offsets: &[i32], face_width: u32) -> Vec<Option<i32>> {
    let bound_cells = bound_cell_offsets;

    let bin_width_in_cells = bound_cells.pairwise().map(|(&a, &b)| b - a);
    let bins_cell_offset = bound_cells.first().unwrap();

    let mut cell_bins: Vec<Option<i32>> = vec![None]; // start with a prepended negative null
    for (bin, width) in bin_width_in_cells.enumerate() {
        // repeat bin, width times
        for _ in 0..width {
            cell_bins.push(Some(bin as i32));
        }
    }
    cell_bins.push(None); // end with an appended positive null

    if *bins_cell_offset < 0 {
        cell_bins =
            cell_bins.iter().skip(bins_cell_offset.wrapping_abs() as usize).cloned().collect();
    } else if *bins_cell_offset > 0 {
        let mut new_bins = vec![None; (*bins_cell_offset) as usize];
        new_bins.extend(cell_bins.iter());
        cell_bins = new_bins;
    }

    if cell_bins.len() < face_width as usize + 2 {
        let deficit = face_width as usize + 2 - cell_bins.len();
        let mut new_bins = cell_bins;
        new_bins.extend(vec![None; deficit].iter());
        cell_bins = new_bins;
    } else if cell_bins.len() > face_width as usize + 2 {
        let new_bins = cell_bins;
        cell_bins = new_bins.iter().take(face_width as usize + 2).cloned().collect();
    }

    cell_bins
}

/// An x-axis label for the text output renderer
#[derive(Debug)]
struct XAxisLabel {
    text: String,
    offset: i32,
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

fn create_x_axis_labels(x_tick_map: &HashMap<i32, f64>) -> Vec<XAxisLabel> {
    let mut ls: Vec<_> = x_tick_map.iter()
        .map(|(&offset, &tick)| {
            XAxisLabel {
                text: tick.to_string(),
                offset: offset,
            }
        })
        .collect();
    ls.sort_by_key(|l| l.offset);
    ls
}

fn render_y_axis_strings(y_axis: &axis::Axis,
                         face_height: u32)
                         -> (Vec<String>, Vec<String>, Vec<String>, i32) {
    // Get the strings and offsets we'll use for the y-axis
    let y_tick_map = tick_offset_map(&y_axis, face_height);

    // Find a minimum size for the left gutter
    let longest_y_label_width = y_tick_map.values()
        .map(|n| n.to_string().len())
        .max()
        .expect("ERROR: There are no y-axis ticks");

    // Generate a list of strings to label the y-axis
    let y_label_strings: Vec<_> = (0..face_height + 1)
        .map(|line| match y_tick_map.get(&(line as i32)) {
            Some(v) => v.to_string(),
            None => "".to_string(),
        })
        .collect();

    // Generate a list of strings to tick the y-axis
    let y_tick_strings: Vec<_> = (0..face_height + 1)
        .map(|line| match y_tick_map.get(&(line as i32)) {
            Some(_) => "-".to_string(),
            None => " ".to_string(),
        })
        .collect();

    // Generate a list of strings to be the y-axis line itself
    let y_axis_line_strings: Vec<String> = std::iter::repeat('+')
        .take(1)
        .chain(std::iter::repeat('|').take(face_height as usize))
        .map(|s| s.to_string())
        .collect();

    (y_label_strings, y_tick_strings, y_axis_line_strings, longest_y_label_width as i32)
}

fn render_x_axis_strings(x_axis: &axis::Axis, face_width: u32) -> (String, String, String, i32) {
    // Get the strings and offsets we'll use for the x-axis
    let x_tick_map = tick_offset_map(x_axis, face_width as u32);

    // Create a string which will be printed to give the x-axis tick marks
    let x_axis_tick_string: String = (0..face_width + 1)
        .map(|cell| match x_tick_map.get(&(cell as i32)) {
            Some(_) => '|',
            None => ' ',
        })
        .collect();

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
    let x_axis_line_string: String = std::iter::repeat('+')
        .take(1)
        .chain(std::iter::repeat('-').take(face_width as usize))
        .collect();

    (x_axis_label_string, x_axis_tick_string, x_axis_line_string, start_offset)
}

/// Given a histogram,
/// the x ands y-axes
/// and the face height and width,
/// create the strings to be drawn as the face
fn render_face_bars(h: &histogram::Histogram, face_width: u32, face_height: u32) -> Vec<String> {
    let bound_cells = bound_cell_offsets(&h, face_width);

    let cell_bins = bins_for_cells(&bound_cells, face_width);

    // counts per bin converted to rows per column
    let cell_heights: Vec<_> = cell_bins.iter()
        .map(|&bin| match bin {
            None => 0,
            Some(b) => {
                value_to_axis_cell_offset(h.bin_counts[b as usize] as f64, &h.y_axis, face_height)
            }
        })
        .collect();

    let mut face_strings: Vec<String> = vec![];

    for line in 1..face_height + 1 {
        let mut line_string = String::new();
        for column in 1..face_width as usize + 1 {
            // maybe use a HashSet for faster `contains()`?
            line_string.push(if bound_cells.contains(&(column as i32)) {
                // The value of the column _below_ this one
                let b = cell_heights[column - 1].cmp(&(line as i32));
                // The value of the column _above_ this one
                let a = cell_heights[column + 1].cmp(&(line as i32));
                match b {
                    std::cmp::Ordering::Less => {
                        match a {
                            std::cmp::Ordering::Less => ' ',
                            std::cmp::Ordering::Equal => '-', // or 'r'-shaped corner
                            std::cmp::Ordering::Greater => '|',
                        }
                    }
                    std::cmp::Ordering::Equal => {
                        match a {
                            std::cmp::Ordering::Less => '-', // or backwards 'r'
                            std::cmp::Ordering::Equal => '-', // or 'T'-shaped
                            std::cmp::Ordering::Greater => '|', // or '-|'
                        }
                    }
                    std::cmp::Ordering::Greater => {
                        match a {
                            std::cmp::Ordering::Less => '|',
                            std::cmp::Ordering::Equal => '|', // or '|-'
                            std::cmp::Ordering::Greater => '|',
                        }
                    }
                }
            } else {
                let bin_height_cells = cell_heights[column];

                if bin_height_cells == line as i32 {
                    '-' // bar cap
                } else {
                    ' ' //
                }
            });
        }
        face_strings.push(line_string);
    }
    face_strings
}

/// Given a scatter plot,
/// the x ands y-axes
/// and the face height and width,
/// create the strings to be drawn as the face
fn render_face_points(s: &scatter::Scatter, face_width: u32, face_height: u32) -> Vec<String> {

    let points: Vec<_> = s.data
        .iter()
        .map(|&(x, y)| {
            (value_to_axis_cell_offset(x, &s.x_axis, face_width),
             value_to_axis_cell_offset(y, &s.y_axis, face_height))
        })
        .collect();

    let mut face_strings: Vec<String> = vec![];
    for line in 1..face_height + 1 {
        let mut line_string = String::new();
        for column in 1..face_width as usize + 1 {
            line_string.push(if points.contains(&(column as i32, line as i32)) {
                'o'
            } else {
                ' '
            });
        }
        face_strings.push(line_string);
    }
    face_strings
}

pub fn draw_histogram(h: &histogram::Histogram) {
    // The face is the actual area of the graph with data on it, excluding axes and labels
    let face_width = 90;
    let face_height = 30u32;

    ////////////
    // Y Axis //
    ////////////

    let (y_label_strings, y_tick_strings, y_axis_line_strings, longest_y_label_width) =
        render_y_axis_strings(&h.y_axis, face_height);

    ////////////
    // X Axis //
    ////////////

    let (x_axis_label_string, x_axis_tick_string, x_axis_line_string, start_offset) =
        render_x_axis_strings(&h.x_axis, face_width);

    //////////
    // Face //
    //////////

    let face_strings = render_face_bars(&h, face_width as u32, face_height);

    /////////////
    // Drawing //
    /////////////

    let left_gutter_width = std::cmp::max(longest_y_label_width as i32 + 1,
                                          start_offset.wrapping_neg()) as
                            usize;

    // Go, line-by-line printing the y-axis and the face
    for line in 0..face_height {
        let cell_position = face_height - line;
        let axis_label = &y_label_strings[cell_position as usize];
        let axis_tick = &y_tick_strings[cell_position as usize];
        let axis_line = &y_axis_line_strings[cell_position as usize];
        let mut axis_label_and_tick = String::new();
        axis_label_and_tick.push_str(axis_label);
        axis_label_and_tick.push_str(axis_tick);
        let face_line = &face_strings[(cell_position as usize) - 1];
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
             plot_width = face_width as usize + 1);

    // Print the x-axis labels
    println!("{:>start_offset$}{: <plot_width$}",
             "",
             x_axis_label_string,
             start_offset = (left_gutter_width as i32 + start_offset) as usize,
             plot_width = (face_width as i32 + 1 + start_offset) as usize);
}

pub fn draw_scatter(s: &scatter::Scatter) {
    let face_width = 90;
    let face_height = 30u32;

    ////////////
    // Y Axis //
    ////////////

    let (y_label_strings, y_tick_strings, y_axis_line_strings, longest_y_label_width) =
        render_y_axis_strings(&s.y_axis, face_height);

    ////////////
    // X Axis //
    ////////////

    let (x_axis_label_string, x_axis_tick_string, x_axis_line_string, start_offset) =
        render_x_axis_strings(&s.x_axis, face_width);

    //////////
    // Face //
    //////////

    let face_strings = render_face_points(&s, face_width as u32, face_height);

    /////////////
    // Drawing //
    /////////////

    let left_gutter_width = std::cmp::max(longest_y_label_width as i32 + 1,
                                          start_offset.wrapping_neg()) as
                            usize;

    // Go, line-by-line printing the y-axis and the face
    for line in 0..face_height {
        let cell_position = face_height - line;
        let axis_label = &y_label_strings[cell_position as usize];
        let axis_tick = &y_tick_strings[cell_position as usize];
        let axis_line = &y_axis_line_strings[cell_position as usize];
        let mut axis_label_and_tick = String::new();
        axis_label_and_tick.push_str(axis_label);
        axis_label_and_tick.push_str(axis_tick);
        let face_line = &face_strings[(cell_position as usize) - 1];
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
             plot_width = face_width as usize + 1);

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
    fn test_bins_for_cells() {
        let face_width = 10;
        let n = i32::max_value();
        let run_bins_for_cells = |bound_cell_offsets: &[i32]| -> Vec<_> {
            bins_for_cells(&bound_cell_offsets, face_width)
                .iter()
                .map(|&a| a.unwrap_or(n))
                .collect()
        };

        assert_eq!(run_bins_for_cells(&vec![-4, -1, 4, 7, 10]),
                   [1, 1, 1, 1, 1, 2, 2, 2, 3, 3, 3, n]);
        assert_eq!(run_bins_for_cells(&vec![0, 2, 4, 8, 10]),
                   [n, 0, 0, 1, 1, 2, 2, 2, 2, 3, 3, n]);
        assert_eq!(run_bins_for_cells(&vec![3, 5, 7, 9, 10]),
                   [n, n, n, n, 0, 0, 1, 1, 2, 2, 3, n]);
        assert_eq!(run_bins_for_cells(&vec![0, 2, 4, 6, 8]),
                   [n, 0, 0, 1, 1, 2, 2, 3, 3, n, n, n]);
        assert_eq!(run_bins_for_cells(&vec![0, 3, 6, 9, 12]),
                   [n, 0, 0, 0, 1, 1, 1, 2, 2, 2, 3, 3]);

        assert_eq!(run_bins_for_cells(&vec![-5, -4, -3, -1, 0]),
                   [3, n, n, n, n, n, n, n, n, n, n, n]);
        assert_eq!(run_bins_for_cells(&vec![10, 12, 14, 16, 18]),
                   [n, n, n, n, n, n, n, n, n, n, n, 0]);

        assert_eq!(run_bins_for_cells(&vec![15, 16, 17, 18, 19]),
                   [n, n, n, n, n, n, n, n, n, n, n, n]);
        assert_eq!(run_bins_for_cells(&vec![-19, -18, -17, -16, -1]),
                   [n, n, n, n, n, n, n, n, n, n, n, n]);
    }

    #[test]
    fn test_value_to_axis_cell_offset() {
        assert_eq!(value_to_axis_cell_offset(3.0, &axis::Axis::new(5.0, 10.0), 10),
                   -4);
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
