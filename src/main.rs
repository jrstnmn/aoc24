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

struct Grid {
    grid: Vec<Vec<char>>,
    count: Vec<Vec<u32>>,
    width: usize,
    height: usize
}

impl Grid {
    fn get(&self, x: i32, y: i32) -> Option<char> {
        if x < 0 || x as usize >= self.width || y < 0 || y as usize >= self.height {
            return None;
        }
        Some(self.grid[y as usize][x as usize])
    }

    fn num_mas(&self, x: i32, y: i32, dx: i32, dy: i32) -> u32 {
        let mas: Vec<char> = vec!['M', 'A', 'S'];

        for i in 0..3 {
            let (dxi, dyi) = (dx * (i + 1), dy * (i + 1));
            if self.get(x + dxi, y + dyi) != Some(mas[i as usize]) {
                return 0;
            }
        }

        1
    }

    fn compute_xmas_count(&mut self, x: i32, y: i32) {
        let dirs: Vec<(i32, i32)> = vec![
            (0, 1), (1, 0), (1, 1), (1, -1),
            (0, -1), (-1, 0), (-1, -1), (-1, 1)
        ];

        let count = dirs.iter().map(|(dx, dy)| {
            self.num_mas(x, y, *dx, *dy)
        }).sum();

        self.count[y as usize][x as usize] = count
    }

    fn compute_crossmas_count(&mut self, x: i32, y: i32) {
        let dirs: Vec<(i32, i32)> = vec![
            (1, 1),
            (-1, 1),
            (-1, -1),
            (1, -1)
        ];

        let char_codes = dirs.iter().map(|(dx, dy)| {
            if (self.get(x - 1, y - 1) == self.get(x + 1, y + 1)) {
                return 0;
            }
            if (self.get(x - 1, y + 1) == self.get(x + 1, y - 1)) {
                return 0;
            }
            self.get(x + *dx, y + *dy)
                .map(|c| c as i32)
                .unwrap_or(0)
        });

        if (char_codes.sum::<i32>() == 320) {
            self.count[y as usize][x as usize] = 1;
        }
    }
}

fn day4_part1() {
    const INPUT: &str = include_str!("input_day4.txt");

    let mut line_length = -1;
    let mut lines: Vec<Vec<char>> = Vec::new();

    for line in INPUT.lines() {
        if (line_length < 0) {
            line_length = line.len() as i32;
        } else {
            assert_eq!(line.len() as i32, line_length);
        }

        lines.push(
            line.chars().collect::<Vec<char>>()
        );
    }

    let num_lines = lines.len();

    let mut grid1 = Grid {
        grid: lines.clone(),
        count: vec![vec![0; line_length as usize]; num_lines],
        width: line_length as usize,
        height: num_lines
    };

    let mut grid2 = Grid {
        grid: lines.clone(),
        count: vec![vec![0; line_length as usize]; num_lines],
        width: line_length as usize,
        height: num_lines
    };

    for y in 0..num_lines as i32 {
        for x in 0..line_length {
            if (grid1.get(x, y) != Some('X')) {
                continue;
            }
            grid1.compute_xmas_count(x, y);
        }
    }

    for y in 0..num_lines as i32 {
        for x in 0..line_length {
            if (grid2.get(x, y) != Some('A')) {
                continue;
            }
            grid2.compute_crossmas_count(x, y);
        }
    }

    let res1 = grid1.count.iter().map(|row| {
        row.iter().sum::<u32>()
    }).sum::<u32>();

    let res2 = grid2.count.iter().map(|row| {
        row.iter().sum::<u32>()
    }).sum::<u32>();

    println!("Day 4 part 1: {}", res1);
    println!("Day 4 part 2: {}", res2);
}

fn main() {
    day4_part1();
}
