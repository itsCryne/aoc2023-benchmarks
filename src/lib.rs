use aoc_macros::day_function_vec;
use humantime::format_duration;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs::read_to_string;
use std::hint::black_box;
use std::process::exit;
use std::str::FromStr;
use std::time::Instant;

pub mod days;

day_function_vec!(DAY_VEC);

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Day(u8);

#[derive(Debug, Copy, Clone)]
pub struct ParseDayError {}
impl Display for ParseDayError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "This does not seem to be a valid AoC day")
    }
}
impl Default for ParseDayError {
    fn default() -> Self {
        Self::new()
    }
}

impl ParseDayError {
    pub fn new() -> Self {
        Self {}
    }
}

impl Error for ParseDayError {}

impl FromStr for Day {
    type Err = ParseDayError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let day_number = s.parse().map_err(|_| ParseDayError::new())?;
        if day_number == 0 || day_number > 25 {
            return Err(ParseDayError::new());
        }
        Ok(Day(day_number))
    }
}

impl Display for Day {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}", self.0)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Days {
    state: usize,
    max: usize,
}

impl Default for Days {
    fn default() -> Self {
        Self::new()
    }
}

impl Days {
    pub fn new() -> Self {
        Days { state: 0, max: 25 }
    }

    pub fn bounded(start: Day, max: Day) -> Self {
        Days {
            state: start.0 as usize - 1,
            max: max.0 as usize,
        }
    }
}

impl Iterator for Days {
    type Item = Day;

    fn next(&mut self) -> Option<Self::Item> {
        self.state += 1;
        if self.state <= self.max {
            Some(Day(self.state as u8))
        } else {
            None
        }
    }
}

impl From<Day> for usize {
    fn from(value: Day) -> Self {
        value.0 as usize - 1
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Part {
    A,
    B,
}

impl From<Part> for usize {
    fn from(value: Part) -> Self {
        match value {
            Part::A => 0,
            Part::B => 1,
        }
    }
}

impl Display for Part {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let string_repr = match &self {
            Part::A => "A",
            Part::B => "B",
        };
        write!(f, "{}", string_repr)
    }
}

pub struct Parts {
    state: usize,
}

impl Parts {
    fn new() -> Self {
        Parts { state: 0 }
    }
}

impl Iterator for Parts {
    type Item = Part;

    fn next(&mut self) -> Option<Self::Item> {
        self.state += 1;
        match self.state {
            1 => Some(Part::A),
            2 => Some(Part::B),
            _ => None,
        }
    }
}

pub fn run_and_print_day(day: Day) {
    let input = match read_to_string(format!("./data/inputs/day_{}.txt", day)) {
        Ok(content) => content,
        Err(why) => {
            eprintln!("Failed to get input for day {}: {}", day, why);
            exit(1);
        }
    };

    match DAY_VEC.get(usize::from(day)) {
        None => {
            eprintln!("Day {} is not yet implemented\n", day);
            exit(1);
        }
        Some(part_fns) => {
            println!("Day {}", day);
            println!("---");
            for part in Parts::new() {
                let part_fn = part_fns[usize::from(part)];

                let start = Instant::now();
                let maybe_result = black_box(part_fn(black_box(input.as_str())));
                let end = start.elapsed();

                match maybe_result {
                    None => println!("Part {}: Not solved", part),
                    Some(result) => {
                        let result_string = result.to_string();
                        if result_string.contains('\n') {
                            println!(
                                "Part {} (t≈{})\n▼▼▼▼▼▼ \n{}",
                                part,
                                format_duration(end),
                                result_string
                            );
                        } else {
                            println!(
                                "Part {}: {} (t≈{})",
                                part,
                                result_string,
                                format_duration(end),
                            )
                        }
                    }
                }
            }
            println!();
        }
    }
}
