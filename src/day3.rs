pub fn day3() {
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
