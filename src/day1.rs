use std::collections::HashMap;
use crate::get_nums;

fn day1_part1() {
    const INPUT: &str = include_str!("input_day1.txt");

    let mut left = Vec::new();
    let mut right = Vec::new();

    INPUT.lines().for_each(|line| {
        let nums = get_nums(line);
        left.push(nums[0]);
        right.push(nums[1]);
    });

    left.sort();
    right.sort();

    let mut tot = 0u32;

    left.iter().zip(right.iter()).for_each(|(l, r)| {
        tot += (l - r).abs() as u32;
    });

    println!("Day 1: {}", tot);
}


fn day1_part2() {
    const INPUT: &str = include_str!("input_day1.txt");

    let mut left = Vec::new();

    let mut occurrences: HashMap<i32, i32> = HashMap::new();

    INPUT.lines().for_each(|line| {
        let nums = get_nums(line);
        left.push(nums[0]);
        match occurrences.get(&nums[1]) {
            Some(n) => occurrences.insert(nums[1], n + 1),
            None => occurrences.insert(nums[1], 1),
        };
    });

    let mut tot = 0i32;

    left.iter().for_each(|(l)| {
        let val = *l * occurrences.get(l).unwrap_or(&0);
        tot += val;
    });

    println!("Day 1: {}", tot);
}

pub fn day1() {
    day1_part1();
    day1_part2();
}