use aoc2021::*;
use std::{env, fs};

macro_rules! run_day {
    ($day:path, $input:expr) => {{
        use $day::{part1, part2};

        println!(
            "{}: part1 = {:?}, part2 = {:?}",
            stringify!($day),
            part1($input),
            part2($input)
        );
    }};
}

pub fn main() {
    let args: Vec<String> = env::args().collect();

    let input = fs::read_to_string(format!("input/2021/day{}.txt", args[1].as_str()))
        .expect("Something went wrong reading the file");
    let input_str: &str = input.as_str();

    let day_num: u8 = args[1].parse().unwrap();

    match day_num {
        1 => run_day!(day1, &input_str),
        _ => println!("Invalid day number: {}", day_num),
    }
}
