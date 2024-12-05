use crate::get_nums;

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

pub fn day2() {
    day2_part1();
    day2_part2();
}