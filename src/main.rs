use std::collections::HashMap;
use std::fmt::Debug;

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

fn get_nums(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .flat_map(|s| match s.parse::<i32>() {
            Ok(n) => Some(n),
            Err(_) => None,
        })
        .collect::<Vec<i32>>()
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

fn safe(level: Vec<i32>) -> bool {
    let mut inc = true;

    for i in 0..level.len() {
        if i == 0 {
            continue;
        }

        if i == 1 {
            inc = level[i] - level[i - 1] > 0;
        }

        let diff = level[i] - level[i - 1];

        if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }

        if (inc && diff < 0) || (!inc && diff > 0) {
            return false;
        }
    }

    true
}

enum Day2Result {
    Safe,
    SafeWithRemoval,
    Unsafe,
}

fn safe2(level: Vec<i32>) -> Day2Result {
    let mut combinations: Vec<Vec<i32>> = Vec::new();
    combinations.push(level.clone());

    for i in 0..level.len() {
        let mut combination = level.clone();
        combination.remove(i);
        combinations.push(combination);
    }

    let mut i = 0;

    for combination in combinations {
        if safe(combination) {
            return if i == 0 {
                Day2Result::Safe
            } else {
                Day2Result::SafeWithRemoval
            };
        }
        i += 1;
    }

    Day2Result::Unsafe
}

fn day2_part1() {
    const INPUT: &str = include_str!("input_day2.txt");

    let mut num_safe = 0;

    INPUT.lines().for_each(|line| {
        let nums = get_nums(line);
        if safe(nums) {
            num_safe += 1;
        }
    });

    println!("Day 2: {}", num_safe);
}

fn day2_part2() {
    const INPUT: &str = include_str!("input_day2.txt");

    let mut num_safe = 0;

    INPUT.lines().for_each(|line| {
        let nums = get_nums(line);
        match safe2(nums) {
            Day2Result::Safe => {
                num_safe += 1;
            }
            Day2Result::SafeWithRemoval => {
                num_safe += 1;
            }
            Day2Result::Unsafe => {}
        }
    });

    println!("Day 2: {}", num_safe);
}

fn day3() {
    const INPUT: &[u8] = include_str!("input_day3.txt").as_bytes();

    enum Mode {
        Prefix,
        Number1,
        Number2,
    }

    const MUL: [u8; 4] = ['m' as u8, 'u' as u8, 'l' as u8, '(' as u8];
    const DO: [u8; 4] = ['d' as u8, 'o' as u8, '(' as u8, ')' as u8];
    const DONT: [u8; 7] = [
        'd' as u8, 'o' as u8, 'n' as u8, '\'' as u8, 't' as u8, '(' as u8, ')' as u8,
    ];

    let mut mode = Mode::Prefix;
    let mut i = 0;
    let mut n1 = 0;
    let mut n2 = 0;
    let mut pairs: Vec<(u32, u32)> = Vec::new();
    let mut skip = false;

    while i < INPUT.len() {
        match mode {
            Mode::Prefix => {
                if i + 4 < INPUT.len() && INPUT[i..i + 4] == DO {
                    (i, skip) = (i + 4, false);
                } else if i + 7 < INPUT.len() && INPUT[i..i + 7] == DONT {
                    (i, skip) = (i + 7, true);
                } else if i + 4 < INPUT.len() && INPUT[i..i + 4] == MUL {
                    i += 4;
                    mode = Mode::Number1;
                } else {
                    i += 1;
                }
            }
            Mode::Number1 => {
                if INPUT[i].is_ascii_digit() {
                    (n1, i) = (10 * n1 + (INPUT[i] - '0' as u8) as u32, i + 1);
                } else if INPUT[i] == ',' as u8 && n1 > 0 {
                    i += 1;
                    mode = Mode::Number2;
                } else {
                    (n1, n2, mode, i) = (0, 0, Mode::Prefix, i + 1);
                }
            }
            Mode::Number2 => {
                if INPUT[i].is_ascii_digit() {
                    (n2, i) = (10 * n2 + (INPUT[i] - '0' as u8) as u32, i + 1);
                } else {
                    if INPUT[i] == ')' as u8 && n2 > 0 && !skip {
                        pairs.push((n1, n2));
                    }
                    (n1, n2, mode, i) = (0, 0, Mode::Prefix, i + 1);
                }
            }
        }
    }

    let mut res = 0;

    pairs.iter().for_each(|(n1, n2)| {
        res += n1 * n2;
    });

    println!("Day 3: {}", res);
}

fn main() {
    day3();
}
