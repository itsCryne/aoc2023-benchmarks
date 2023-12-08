use clap::{Parser, Subcommand};
use std::fs::File;
use std::io::{ErrorKind, Write};

use advent_of_code_rust_criterion::{run_and_print_day, Day, Days};
use aoc_client::{AocClient, AocResult, PuzzleDay};
use std::process::exit;

macro_rules! day_content {
    ($day:ident) => {
        format!(
            r#"pub fn part_a(input: &str) -> Option<u32> {{
    None
}}

pub fn part_b(input: &str) -> Option<u32> {{
    None
}}

#[cfg(test)]
mod tests {{
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_day_0{}_part_a() {{
        let input_a = read_to_string("./data/examples/day_{}_a.txt").unwrap();
        let result = part_a(input_a.as_str());
        assert_eq!(result, None);
    }}

    #[test]
    fn test_day_0{}_part_b() {{
        let input_b = read_to_string("./data/examples/day_{}_b.txt").unwrap();
        let result = part_b(input_b.as_str());
        assert_eq!(result, None);
    }}
}}"#,
            $day, $day, $day, $day
        )
    };
}

#[derive(Parser)]
struct AoC {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Solve { day: Day },
    SolveMultiple { start: Day, end: Day },
    SolveAll,
    Initialize { day: Day },
    Download { day: Day },
}

fn main() -> AocResult<()> {
    let args = AoC::parse();
    match args.command {
        Command::Solve { day } => {
            run_and_print_day(day);
        }
        Command::SolveMultiple { start, end } => {
            for day in Days::bounded(start, end) {
                run_and_print_day(day);
            }
        }
        Command::SolveAll => {
            for day in Days::new() {
                run_and_print_day(day);
            }
        }
        Command::Initialize { day } => {
            match File::options()
                .write(true)
                .create_new(true)
                .open(format!("./src/days/day_{}.rs", day).as_str())
            {
                Ok(mut file) => {
                    if let Err(why) = file.write(day_content!(day).as_bytes()) {
                        eprintln!("Could not initialize day {}: {}", day, why);
                        exit(1);
                    }
                }
                Err(why) => match why.kind() {
                    ErrorKind::AlreadyExists => {
                        println!("Day {} is already initialized", day);
                        exit(0);
                    }
                    _ => {
                        eprintln!("Could not initialize day {}: {}", day, why);
                        exit(1);
                    }
                },
            }

            match File::options()
                .write(true)
                .create_new(true)
                .open(format!("./data/examples/day_{}_a.txt", day).as_str())
            {
                Ok(_) => {}
                Err(why) => match why.kind() {
                    ErrorKind::AlreadyExists => {
                        println!("Day {} is already initialized", day);
                        exit(0);
                    }
                    _ => {
                        eprintln!("Could not initialize example file for day {}. Please remove already generated and now orphaned day file manually. {}", day, why);
                        exit(1);
                    }
                },
            }

            match File::options()
                .write(true)
                .create_new(true)
                .open(format!("./data/examples/day_{}_b.txt", day).as_str())
            {
                Ok(_) => {}
                Err(why) => match why.kind() {
                    ErrorKind::AlreadyExists => {
                        println!("Day {} is already initialized", day);
                        exit(0);
                    }
                    _ => {
                        eprintln!("Could not initialize example file for day {}. Please remove already generated and now orphaned day file manually. {}", day, why);
                        exit(1);
                    }
                },
            }

            match File::options().append(true).open("./src/days.rs") {
                Ok(mut file) => {
                    if let Err(why) = file.write(format!("\npub mod day_{};", day).as_bytes()) {
                        eprintln!(
                            "Could not add module for Day {}. Please remove already generated and now orphaned day & example files manually. {}",
                            day, why
                        );
                        exit(1);
                    }
                    println!("Initialized Day {}", day);
                }
                Err(why) => {
                    eprintln!(
                        "Could not add module for Day {}. Please remove already generated and now orphaned day & example files manually. {}",
                        day, why
                    );
                    exit(1);
                }
            }
        }
        Command::Download { day } => {
            let client = AocClient::builder()
                .session_cookie_from_default_locations()?
                .year(std::env::var("AOC_YEAR").unwrap().parse().unwrap())?
                .day((usize::from(day) + 1) as PuzzleDay)?
                .build()?;
            let input = client.get_input()?;

            match File::options()
                .write(true)
                .create_new(true)
                .open(format!("./data/inputs/day_{}.txt", day).as_str())
            {
                Ok(mut file) => {
                    if let Err(why) = file.write(input.as_bytes()) {
                        eprintln!("Could not download Day {}: {}", day, why);
                        exit(1);
                    }
                    println!("Downloaded Day {}", day);
                }
                Err(why) => match why.kind() {
                    ErrorKind::AlreadyExists => {
                        println!("Day {} is already downloaded", day);
                        exit(0);
                    }
                    _ => {
                        eprintln!("Could not download day {}: {}", day, why);
                        exit(1);
                    }
                },
            }
        }
    }

    Ok(())
}
