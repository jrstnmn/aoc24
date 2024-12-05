use std::collections::{HashMap, HashSet};

fn valid_update(update: &Vec<u32>, rules: &HashMap<u32, Vec<u32>>) -> bool {
    let mut existing: HashSet<u32> = HashSet::new();
    let mut seen: HashSet<u32> = HashSet::new();

    for n in update {
        existing.insert(*n);
    }

    for n in update {
        seen.insert(*n);
        if let Some(should_be_seen) = rules.get(n) {
            for n0 in should_be_seen {
                if existing.contains(n0) && !seen.contains(n0) {
                    return false;
                }
            }
        }
    }

    true
}

struct RuleSet {
    rules: HashMap<u32, Vec<u32>>
}

impl RuleSet {
    pub fn reorder(&self, update: &mut Vec<u32>) {
        let len = update.len();
        for i in (0..len).rev() {
            for j in (0..i).rev() {
                if let Some(left) = self.rules.get(&update[i]) {
                    if left.contains(&update[j]) {
                        update.swap(i, j);
                    }
                }
            }

        }
    }
}

pub fn day5() {
    const INPUT : &str = include_str!("input_day5.txt");

    enum Mode {
        Rules,
        Updates
    }

    let mut mode = Mode::Rules;

    let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut updates: Vec<Vec<u32>> = Vec::new();

    for line in INPUT.lines() {
        match mode {
            Mode::Rules => {
                if !line.is_empty() {
                    if let Some((l,r )) = line.split_once('|') {
                        if let (Ok(li), Ok(ri)) = (l.parse::<u32>(), r.parse::<u32>()) {
                            if !rules.contains_key(&ri) {
                                rules.insert(ri, Vec::new());
                            }
                            let pages = rules.get_mut(&ri).unwrap();
                            pages.push(li);
                        }
                    }
                } else {
                    mode = Mode::Updates;
                }
            }
            Mode::Updates => {
                if !line.is_empty() {
                    let nums = line.split(',').map(|s| s.parse::<u32>().unwrap()).collect();
                    updates.push(nums);
                }
            }
        }
    }

    let rule_set = RuleSet {
        rules: rules.clone(),
    };

    let mut res1 = 0;
    let mut res2 = 0;
    for mut update in updates.iter_mut() {
        if valid_update(update, &rules) {
            let len = update.len();
            let mid = len / 2;
            res1 += update[mid];
        }
        else {
            rule_set.reorder(&mut update);
            let len = update.len();
            let mid = len / 2;
            res2 += update[mid];
        }
    }

    println!("Day 5 part 1: {}", res1);
    println!("Day 5 part 2: {}", res2);
}
