use std::iter::FromIterator;

#[derive(Debug)]
pub struct Axis {
    lower: f64,
    upper: f64,
    ticks: Vec<f64>,
}

impl Axis {
    pub fn new(lower: f64, upper: f64) -> Axis {
        assert!(lower < upper);
        let default_max_ticks = 6;
        Axis {
            lower: lower,
            upper: upper,
            ticks: calculate_ticks(lower, upper, default_max_ticks),
        }
    }

    pub fn max(&self) -> f64 {
        self.upper
    }

    pub fn min(&self) -> f64 {
        self.lower
    }

    pub fn ticks(&self) -> &Vec<f64> {
        &self.ticks
    }
}

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
    ticks.sort_by(|a, b| a.partial_cmp(b).expect("ERROR: Invalid tick value found"));
    ticks
}

/// Given a range and a step size, work out how many ticks will be displayed
fn number_of_ticks(min: f64, max: f64, step_size: f64) -> u32 {
    generate_ticks(min, max, step_size).len() as u32
}

/// Given a range of values, and a maximum number of ticks, calulate the step between the ticks
fn calculate_tick_step_for_range(min: f64, max: f64, max_ticks: u32) -> f64 {
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

/// Given an axis range, calculate the sensible places to place the ticks
fn calculate_ticks(min: f64, max: f64, max_ticks: u32) -> Vec<f64> {
    let tick_step = calculate_tick_step_for_range(min, max, max_ticks);
    generate_ticks(min, max, tick_step)
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_calculate_tick_step_for_range() {
        assert_eq!(calculate_tick_step_for_range(0.0, 6.0, 6), 2.0);
        assert_eq!(calculate_tick_step_for_range(0.0, 11.0, 6), 2.0);
        assert_eq!(calculate_tick_step_for_range(0.0, 14.0, 6), 4.0);
        assert_eq!(calculate_tick_step_for_range(0.0, 15.0, 6), 5.0);
        assert_eq!(calculate_tick_step_for_range(-1.0, 5.0, 6), 2.0);
        assert_eq!(calculate_tick_step_for_range(-7.93, 15.58, 6), 5.0);
    }

    #[test]
    fn test_calculate_ticks() {
        //assert_eq!(calculate_ticks(1), [0, 1]); // step up in 1s
        assert_eq!(calculate_ticks(0.0, 2.0, 6), [0.0, 1.0, 2.0]);
        assert_eq!(calculate_ticks(0.0, 3.0, 6), [0.0, 1.0, 2.0, 3.0]);
        assert_eq!(calculate_ticks(0.0, 4.0, 6), [0.0, 1.0, 2.0, 3.0, 4.0]);
        assert_eq!(calculate_ticks(0.0, 5.0, 6), [0.0, 1.0, 2.0, 3.0, 4.0, 5.0]);
        assert_eq!(calculate_ticks(0.0, 6.0, 6), [0.0, 2.0, 4.0, 6.0]);
        assert_eq!(calculate_ticks(0.0, 7.0, 6), [0.0, 2.0, 4.0, 6.0]);
        assert_eq!(calculate_ticks(0.0, 8.0, 6), [0.0, 2.0, 4.0, 6.0, 8.0]);
        assert_eq!(calculate_ticks(0.0, 9.0, 6), [0.0, 2.0, 4.0, 6.0, 8.0]);
        assert_eq!(calculate_ticks(0.0, 10.0, 6),
                   [0.0, 2.0, 4.0, 6.0, 8.0, 10.0]);
        assert_eq!(calculate_ticks(0.0, 11.0, 6),
                   [0.0, 2.0, 4.0, 6.0, 8.0, 10.0]);
        assert_eq!(calculate_ticks(0.0, 12.0, 6), [0.0, 4.0, 8.0, 12.0]);
        assert_eq!(calculate_ticks(0.0, 13.0, 6), [0.0, 4.0, 8.0, 12.0]);
        assert_eq!(calculate_ticks(0.0, 14.0, 6), [0.0, 4.0, 8.0, 12.0]);
        assert_eq!(calculate_ticks(0.0, 15.0, 6), [0.0, 5.0, 10.0, 15.0]);
        assert_eq!(calculate_ticks(0.0, 16.0, 6), [0.0, 4.0, 8.0, 12.0, 16.0]);
        assert_eq!(calculate_ticks(0.0, 17.0, 6), [0.0, 4.0, 8.0, 12.0, 16.0]);
        assert_eq!(calculate_ticks(0.0, 18.0, 6), [0.0, 4.0, 8.0, 12.0, 16.0]);
        assert_eq!(calculate_ticks(0.0, 19.0, 6), [0.0, 4.0, 8.0, 12.0, 16.0]);
        assert_eq!(calculate_ticks(0.0, 20.0, 6),
                   [0.0, 4.0, 8.0, 12.0, 16.0, 20.0]);
        assert_eq!(calculate_ticks(0.0, 21.0, 6),
                   [0.0, 4.0, 8.0, 12.0, 16.0, 20.0]);
        assert_eq!(calculate_ticks(0.0, 22.0, 6),
                   [0.0, 4.0, 8.0, 12.0, 16.0, 20.0]);
        assert_eq!(calculate_ticks(0.0, 23.0, 6),
                   [0.0, 4.0, 8.0, 12.0, 16.0, 20.0]);
        assert_eq!(calculate_ticks(0.0, 24.0, 6), [0.0, 5.0, 10.0, 15.0, 20.0]);
        assert_eq!(calculate_ticks(0.0, 25.0, 6),
                   [0.0, 5.0, 10.0, 15.0, 20.0, 25.0]);
        assert_eq!(calculate_ticks(0.0, 26.0, 6),
                   [0.0, 5.0, 10.0, 15.0, 20.0, 25.0]);
        assert_eq!(calculate_ticks(0.0, 27.0, 6),
                   [0.0, 5.0, 10.0, 15.0, 20.0, 25.0]);
        assert_eq!(calculate_ticks(0.0, 28.0, 6),
                   [0.0, 5.0, 10.0, 15.0, 20.0, 25.0]);
        assert_eq!(calculate_ticks(0.0, 29.0, 6),
                   [0.0, 5.0, 10.0, 15.0, 20.0, 25.0]);
        assert_eq!(calculate_ticks(0.0, 30.0, 6), [0.0, 10.0, 20.0, 30.0]);
        assert_eq!(calculate_ticks(0.0, 31.0, 6), [0.0, 10.0, 20.0, 30.0]);
        //...
        assert_eq!(calculate_ticks(0.0, 40.0, 6), [0.0, 10.0, 20.0, 30.0, 40.0]);
        assert_eq!(calculate_ticks(0.0, 50.0, 6),
                   [0.0, 10.0, 20.0, 30.0, 40.0, 50.0]);
        assert_eq!(calculate_ticks(0.0, 60.0, 6), [0.0, 20.0, 40.0, 60.0]);
        assert_eq!(calculate_ticks(0.0, 70.0, 6), [0.0, 20.0, 40.0, 60.0]);
        assert_eq!(calculate_ticks(0.0, 80.0, 6), [0.0, 20.0, 40.0, 60.0, 80.0]);
        assert_eq!(calculate_ticks(0.0, 90.0, 6), [0.0, 20.0, 40.0, 60.0, 80.0]);
        assert_eq!(calculate_ticks(0.0, 100.0, 6),
                   [0.0, 20.0, 40.0, 60.0, 80.0, 100.0]);
        assert_eq!(calculate_ticks(0.0, 110.0, 6),
                   [0.0, 20.0, 40.0, 60.0, 80.0, 100.0]);
        assert_eq!(calculate_ticks(0.0, 120.0, 6), [0.0, 40.0, 80.0, 120.0]);
        assert_eq!(calculate_ticks(0.0, 130.0, 6), [0.0, 40.0, 80.0, 120.0]);
        assert_eq!(calculate_ticks(0.0, 140.0, 6), [0.0, 40.0, 80.0, 120.0]);
        assert_eq!(calculate_ticks(0.0, 150.0, 6), [0.0, 50.0, 100.0, 150.0]);
        //...
        assert_eq!(calculate_ticks(0.0, 3475.0, 6),
                   [0.0, 1000.0, 2000.0, 3000.0]);
    }
}
