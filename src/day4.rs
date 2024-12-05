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

pub fn day4() {
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
