use std::error::Error;
use std::io::{self, Read, Write};
use structopt::StructOpt;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
#[cfg(test)]
mod testmacros;

struct Solution {
    part1: fn(&str) -> Result<String, Box<dyn Error + '_>>,
    part2: fn(&str) -> Result<String, Box<dyn Error + '_>>,
}

const SOLUTIONS: &[Solution] = &[
    day1::DAY1,
    day2::DAY2,
    day3::DAY3,
    day4::DAY4,
    day5::DAY5,
    day6::DAY6,
    day7::DAY7,
    day8::DAY8,
];

#[derive(StructOpt)]
struct Options {
    /// Day for which a solution should be ran
    day: u8,
    /// Input, if not provided taken from stdin
    input: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Options::from_args();
    let solution = SOLUTIONS
        .get(usize::from(opt.day) - 1)
        .ok_or("Day number out of range")?;
    let input = if let Some(input) = opt.input {
        input
    } else {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input)?;
        input
    };
    writeln!(
        io::stdout(),
        "Part 1: {}",
        (solution.part1)(&input).map_err(|e| e.to_string())?
    )?;
    writeln!(
        io::stdout(),
        "Part 2: {}",
        (solution.part2)(&input).map_err(|e| e.to_string())?
    )?;
    Ok(())
}
