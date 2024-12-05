mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

use crate::day1::day1;
use crate::day2::day2;
use crate::day3::day3;
use crate::day4::day4;
use crate::day5::day5;

fn get_nums(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .flat_map(|s| match s.parse::<i32>() {
            Ok(n) => Some(n),
            Err(_) => None,
        })
        .collect::<Vec<i32>>()
}

fn main() {
    day5();
}
