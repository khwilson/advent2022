use std::cmp::Ordering;
use std::collections::{HashSet, HashMap, VecDeque};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::u32::MAX;
use std::vec;
use itertools::Itertools;
use serde_json::{Value};
use lazy_static::lazy_static;
use regex::Regex;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn posmod(x: i32, m: i32) -> i32 {
    ((x % m) + m) % m
}

fn day01() {
    let mut totals: Vec<i32> = vec!();
    let mut cur_total: i32 = 0;
    if let Ok(lines) = read_lines("./input/input01") {
        for line in lines {
            if let Ok(ip) = line {
                let tip = ip.trim();
                if tip.len() == 0 {
                    totals.push(cur_total);
                    cur_total = 0;
                } else {
                    let val: i32 = tip.parse().unwrap();
                    cur_total += val;
                }
            }
        }
    }
    totals.push(cur_total);
    let out = totals.iter().enumerate().fold((0, 0), |(oi, ov), (i, v)| if *v > ov { (i, *v) } else { (oi, ov) });
    println!("The answer to part 1 is: {}", out.1);

    totals.sort();
    totals.reverse();
    let out2 = totals[0] + totals[1] + totals[2];
    println!("The answer to part 2 is: {}", out2);
}

fn day02() {
    let mut total_score: i32 = 0;
    if let Ok(lines) = read_lines("./input/input02") {
        for line in lines {
            if let Ok(ip) = line {
                let uleft: u32 = ip.chars().nth(0).unwrap().into();
                let uright: u32 = ip.chars().nth(2).unwrap().into();
                let left: i32 = i32::try_from(uleft).ok().unwrap() - 0x41;
                let right: i32 = i32::try_from(uright).ok().unwrap() - 0x58;
                let score = 1 + right + posmod(right - left + 1, 3) * 3;
                total_score += score;
            }
        }
    }
    println!("The answer to part 1 is: {}", total_score);

    total_score = 0;
    if let Ok(lines) = read_lines("./input/input02") {
        for line in lines {
            if let Ok(ip) = line {
                let uleft: u32 = ip.chars().nth(0).unwrap().into();
                let uright: u32 = ip.chars().nth(2).unwrap().into();
                let left: i32 = i32::try_from(uleft).ok().unwrap() - 0x41;
                let tright: i32 = i32::try_from(uright).ok().unwrap() - 0x58;

                let right: i32 = posmod(left + (tright - 1), 3);

                let score = 1 + right + posmod(right - left + 1, 3) * 3;
                total_score += score;
            }
        }
    }
    println!("The answer to part 2 is: {}", total_score);
}

fn get_priority(c: char) -> i32 {
    let uc: u32 = c.into();
    let lc = c.to_lowercase().nth(0).unwrap();

    let ulc: u32 = lc.into();
    let ilc: i32 = i32::try_from(ulc).ok().unwrap();

    1 + (ilc - 0x61) + 26 * (if uc <= (0x40 + 26) { 1 } else { 0 })
}

fn day03() {
    let mut total = 0;
    if let Ok(lines) = read_lines("./input/input03") {
        for line in lines {
            if let Ok(ip) = line {
                let tip = ip.trim();

                let tip_left = &tip[0..(tip.len()/2)];
                let tip_right = &tip[(tip.len()/2)..tip.len()];

                let left_set: HashSet<i32> = tip_left.chars().map(get_priority).collect();
                let right_set: HashSet<i32> = tip_right.chars().map(get_priority).collect();

                let intersection = left_set.intersection(&right_set);
                let val = intersection.into_iter().nth(0).unwrap();
                total += val;
            }
        }
    }
    println!("The answer the part 1 is: {}", total);

    let mut total = 0;
    if let Ok(lines) = read_lines("./input/input03") {
        for (a, b, c) in lines.tuples() {
            let ia = a.unwrap();
            let ib = b.unwrap();
            let ic = c.unwrap();
            let tia: HashSet<i32> = ia.trim().chars().map(get_priority).collect();
            let tib: HashSet<i32> = ib.trim().chars().map(get_priority).collect();
            let tic: HashSet<i32> = ic.trim().chars().map(get_priority).collect();

            let mut s: HashSet<i32> = tia.intersection(&tib).copied().collect();
            s = s.intersection(&tic).copied().collect();

            let val = s.into_iter().nth(0).unwrap();
            total += val;
        }
    }

    println!("The answer the part 2 is: {}", total);
}

fn day04() {
    // Full overlaps
    let mut total1 = 0;
    let mut total2 = 0;
    if let Ok(lines) = read_lines("./input/input04") {
        for line in lines {
            if let Ok(ip) = line {
                let mut tip = ip.trim().split(",");
                let a = tip.next().unwrap();
                let b = tip.next().unwrap();

                let mut ai = a.split("-");
                let al: i32 = ai.next().unwrap().parse().unwrap();
                let ar: i32 = ai.next().unwrap().parse().unwrap();

                let mut bi = b.split("-");
                let bl: i32 = bi.next().unwrap().parse().unwrap();
                let br: i32 = bi.next().unwrap().parse().unwrap();

                // Full overlap
                if (al <= bl && br <= ar) || (bl <= al && ar <= br) {
                    total1 += 1;
                }

                // Any overlap
                if (al <= bl && bl <= ar) || (al <= br && br <= ar) ||
                   (bl <= al && al <= br) || (bl <= ar && ar <= br) {
                    total2 += 1;
                }
            }
        }
    }
    println!("The answer the part 1 is: {}", total1);
    println!("The answer the part 2 is: {}", total2);
}

fn day05() {
    let mut reading_original_position = true;
    let mut positions1: Vec<Vec<char>> = vec!();
    let mut positions2: Vec<Vec<char>> = vec!();
    if let Ok(lines) = read_lines("./input/input05") {
        for line in lines {
            if let Ok(ip) = line {
                let tip = ip.trim_end();
                if tip.len() == 0 {
                    reading_original_position = false;
                    // Pop the last element and reverse the vecs
                    for i in 0..positions1.len() {
                        positions1[i].reverse();
                        positions2[i].reverse();
                    }
                    continue;
                }

                if reading_original_position {
                    let num_cols = (tip.len() + 1) / 4;

                    // Initialize data now that we know its size
                    while positions1.len() < num_cols {
                        positions1.push(vec!());
                        positions2.push(vec!());
                    }

                    // Extract the data (assumes well formatted)
                    for i in 0..num_cols {
                        let c = tip.chars().nth(4 * i + 1).unwrap();
                        if c.is_ascii_alphabetic() {
                            positions1[i].push(c);
                            positions2[i].push(c);
                        }
                    }
                } else {
                    // Now we get a bunch of lines like:
                    // move X from Y to Z so just break on spaces
                    let mut vals = tip.split(" ");
                    let x: usize = vals.nth(1).unwrap().parse().unwrap();
                    let y: usize = vals.nth(1).unwrap().parse().unwrap();
                    let z: usize = vals.nth(1).unwrap().parse().unwrap();

                    // Actually move the data
                    for _ in 0..x {
                        let elt = positions1[y - 1].pop().unwrap();
                        positions1[z - 1].push(elt);
                    }

                    // Part 2's alternate moving strategy
                    let mut tmp: Vec<char> = vec!();
                    for _ in 0..x {
                        let elt = positions2[y - 1].pop().unwrap();
                        tmp.push(elt);
                    }
                    for _ in 0..x {
                        let elt = tmp.pop().unwrap();
                        positions2[z - 1].push(elt);
                    }
                }
            }
        }
    }
    print!("The answer to part 1 is: ");
    for i in 0..positions1.len() {
        print!("{}", positions1[i].last().unwrap());
    }
    println!();

    print!("The answer to part 2 is: ");
    for i in 0..positions2.len() {
        print!("{}", positions2[i].last().unwrap());
    }
    println!();
}

fn day06() {
    if let Ok(lines) = read_lines("./input/input06") {
        for line in lines {
            if let Ok(ip) = line {
                let tip = ip.trim_end();
                let ans1 = tip.chars().collect_vec()
                    .windows(4)
                    .position(
                        |x| x.iter().copied().collect::<HashSet<char>>().len() == 4
                    ).unwrap();
                let ans2 = tip.chars().collect_vec()
                    .windows(14)
                    .position(
                        |x| x.iter().copied().collect::<HashSet<char>>().len() == 14
                    ).unwrap();

                println!("The answer to part 1 is: {}", ans1 + 4);
                println!("The answer to part 2 is: {}", ans2 + 14);
            }
        }
    }
}

enum ReadState {
    COMMAND = 0,
    LS = 1,
}

struct Node {
    parent_idx: usize,
    idx: usize,
    child_idxs: Vec<usize>,

    size: u32,

    is_dir: bool,
    is_file: bool,
}

fn get_node_size(cache: &mut HashMap<usize, u32>, node_idx: usize, nodes: &Vec<Node>) -> u32 {
    if nodes[node_idx].is_file {
        return nodes[node_idx].size;
    }
    match cache.get(&node_idx) {
        Some(size) => { *size },
        None => {
            let size: u32 = nodes[node_idx].child_idxs.iter().map(|x| get_node_size(cache, *x, nodes)).sum();
            cache.insert(node_idx, size);
            size
        }
    }
}

fn day07() {
    let mut read_state = ReadState::COMMAND;

    let root = Node {
        parent_idx: 0,
        idx: 0,
        child_idxs: vec![],
        size: 0,
        is_dir: true,
        is_file: false,
    };
    let mut nodes: Vec<Node> = vec![root];
    let mut current_node_idx: usize = 0;

    if let Ok(lines) = read_lines("./input/input07") {
        for line in lines {
            if let Ok(ip) = line {
                let tip = ip.trim_end();
                let stip = tip.split(" ").collect_vec();

                if stip[0] == "$" {
                    read_state = ReadState::COMMAND;
                }

                match read_state {
                    ReadState::COMMAND => {
                        match stip[1] {
                            "cd" => {
                                match stip[2] {
                                    "/" => {
                                        current_node_idx = 0;
                                    },
                                    ".." => {
                                        current_node_idx = nodes[current_node_idx].parent_idx;
                                    },
                                    _ => {
                                        let idx = nodes.len();
                                        let this_node = Node {
                                            parent_idx: current_node_idx,
                                            idx,
                                            child_idxs: vec![],
                                            size: 0,
                                            is_dir: true,
                                            is_file: false,
                                        };
                                        nodes[current_node_idx].child_idxs.push(idx);
                                        current_node_idx = idx;
                                        nodes.push(this_node);
                                    }
                                }
                            },
                            "ls" => {
                                read_state = ReadState::LS;
                            },
                            _ => panic!("This shouldn't happen"),
                        }
                    },
                    ReadState::LS => {
                        if stip[0] == "dir" {
                            // We don't actually care about dirs
                        } else {
                            let file_size: u32 = stip[0].parse().unwrap();
                            let idx = nodes.len();
                            let this_node = Node {
                                parent_idx: current_node_idx,
                                idx,
                                child_idxs: vec![],
                                size: file_size,
                                is_dir: false,
                                is_file: true,
                            };
                            nodes[current_node_idx].child_idxs.push(idx);
                            nodes.push(this_node);
                        }
                    },
                }
            }
        }
    }
    let mut cache = HashMap::new();
    let dir_sizes: Vec<u32> = nodes.iter()
        .filter(|node| node.is_dir)
        .map(|node| get_node_size(&mut cache, node.idx, &nodes))
        .collect();

    let ans1: u32 = dir_sizes.iter().filter(|x| **x <= 100000).sum();
    println!("The answer to part 1 is: {}", ans1);

    let total_space: u32 = 70000000;
    let need_space: u32 = 30000000;
    let used_space = get_node_size(&mut cache, 0, &nodes);
    let avail_space = total_space - used_space;
    let find_space: u32 = need_space - avail_space;

    let ans2 = dir_sizes.iter().filter(|size| **size >= find_space).min().unwrap();
    println!("The answer to part 2 is: {}", ans2);
}

fn day08() {
    // Read in the trees
    let mut trees: Vec<Vec<i8>> = vec![];
    if let Ok(lines) = read_lines("./input/input07") {
        for line in lines {
            if let Ok(ip) = line {
                let tip = ip.trim_end();
                let row: Vec<i8> = tip.chars().map(|x| i8::try_from(u32::try_from(x).unwrap()).unwrap()).collect();
                trees.push(row);
            }
        }
    }
    // let num_rows = trees.len();
    // let num_cols = trees[0].len();

    // let mut viz_from_left: Vec<Vec<(i8, bool)>> = vec![];
    // let mut viz_from_right: Vec<VecDeque<(i8, bool)>> = vec![];
    // let mut viz_from_top: Vec<Vec<bool>> = vec![];
    // let mut viz_from_bottom: Vec<Vec<bool>> = vec![];

    // for i in 0..num_rows {
    //     viz_from_left.push(vec![(trees[i][0], true)]);
    //     for j in 1..num_cols {
    //         let p = viz_from_left[i][j - 1].0;
    //         let v = trees[i][j];
    //         let m = max(p, v);
    //         viz_from_left[i].push((m, v > p));
    //     }
    // }

    // for i in 0..num_rows {
    //     viz_from_right.push(VecDeque::new());
    //     viz_from_right[i].push_back((trees[i][num_cols - 1], true));
    //     for j in 2..(num_cols + 1) {
    //         let p = viz_from_right[i][0].0;
    //         let v = trees[i][num_cols - j];
    //         let m = max(p, v);
    //         viz_from_right[i].push_front((m, v > p));
    //     }
    // }
}

fn move_me(xs: &mut Vec<i32>, ys: &mut Vec<i32>, amt: i32) {
    xs[0] += amt;
    for i in 1..xs.len() {
        let xdist = (xs[i] - xs[i - 1]).abs();
        let ydist = (ys[i] - ys[i - 1]).abs();
        if (xdist <= 1) && (ydist <= 1) {
            // They touch; nothing to do
            return;
        }
        if xdist == 2 {
            xs[i] = xs[i - 1] + (if xs[i - 1] < xs[i] { 1 } else { -1 });
        } else {
            xs[i] = xs[i - 1];
        }
        if ydist == 2 {
            ys[i] = ys[i - 1] + (if ys[i - 1] < ys[i] { 1 } else { -1 });
        } else {
            ys[i] = ys[i - 1];
        }
    }
}

fn day09_helper(num_knots: usize) -> usize {
    let mut xs: Vec<i32> = vec![];
    let mut ys: Vec<i32> = vec![];
    for _ in 0..num_knots {
        xs.push(0);
        ys.push(0);
    }

    let mut tail_history: Vec<(i32, i32)> = vec![(0, 0)];

    if let Ok(lines) = read_lines("./input/input09") {
        for line in lines {
            if let Ok(ip) = line {
                let stip = ip.trim_end().split(" ").collect_vec();
                let dir = stip[0];
                let amt: i32 = stip[1].parse().unwrap();

                for _ in 0..amt {
                    match dir {
                        "R" => move_me(&mut xs, &mut ys, 1),
                        "L" => move_me(&mut xs, &mut ys, -1),
                        "U" => move_me(&mut ys, &mut xs, 1),
                        "D" => move_me(&mut ys, &mut xs, -1),
                        _ => panic!("Can't happen"),
                    }
                    tail_history.push((*xs.last().unwrap(), *ys.last().unwrap()));
                };
            }
        }
    }
    let t: HashSet<_> = tail_history.into_iter().collect();
    t.len()
}

fn day09() {
    println!("The answer to part 1 is: {}", day09_helper(2));
    println!("The answer to part 2 is: {}", day09_helper(10));
}

fn day10() {
    if let Ok(lines) = read_lines("./input/input10") {
        let mut reg_vals: Vec<i32> = lines.map(|line| if let Ok(ip) = line {
            let mut stip = ip.trim_end().split(" ");
            match stip.nth(0).unwrap() {
                "noop" => vec![0],
                "addx" => {
                    let val: i32 = stip.nth(0).unwrap().parse().unwrap();
                    vec![0, val]
                },
                _ => panic!("Can't happen"),
            }
        } else { vec![] })
        .flatten()
        .scan(1, |acc, x| {
            *acc = *acc + x;
            Some(*acc)
        }).collect();

        let mut new_reg_vals = vec![1, 1];
        new_reg_vals.append(&mut reg_vals);
        let total: i32 = (0..6).map(|x| x * 40 + 20).map(|x| new_reg_vals[x] * i32::try_from(x).unwrap()).sum();
        println!("The answer to part 1 is: {}", total);

        print!("The answer to part 2 is:");
        for i in 1..241 {
            let x: i32 = i32::try_from(i - 1).unwrap() % 40;
            if x == 0 {
                println!();
            }
            if ((new_reg_vals[i] - 1) <=  x) && ((new_reg_vals[i] + 1) >= x) {
                print!("#");
            } else {
                print!(" ");
            }
        }
    }
    println!();
}

struct Monkey {
    items: Vec<i64>,
    operation: fn(i64) -> i64,
    test: fn(i64) -> bool,
    true_monkey: usize,
    false_monkey: usize,
}

fn get_monkeys() -> Vec<Monkey> {
    vec![
        Monkey {
            items: vec![66, 79],
            operation: (|old| old * 11),
            test: (|val| (val % 7) == 0),
            true_monkey: 6,
            false_monkey: 7,
        },
        Monkey {
            items: vec![84, 94, 94, 81, 98, 75],
            operation: |old| old * 17,
            test: |val| (val % 13) == 0,
            true_monkey: 5,
            false_monkey: 2,
        },
        Monkey {
            items: vec![85, 79, 59, 64, 79, 95, 67],
            operation: |old| old + 8,
            test: |val| (val % 5) == 0,
            true_monkey: 4,
            false_monkey: 5,
        },
        Monkey {
            items: vec![70],
            operation: |old| old + 3,
            test: |val| (val % 19) == 0,
            true_monkey: 6,
            false_monkey: 0,
        },
        Monkey {
            items: vec![57, 69, 78, 78],
            operation: |old| old + 4,
            test: |val| (val % 2) == 0,
            true_monkey: 0,
            false_monkey: 3,
        },
        Monkey {
            items: vec![65, 92, 60, 74, 72],
            operation: |old| old + 7,
            test: |val| (val % 11) == 0,
            true_monkey: 3,
            false_monkey: 4,
        },
        Monkey {
            items: vec![77, 91, 91],
            operation: |old| old * old,
            test: |val| (val % 17) == 0,
            true_monkey: 1,
            false_monkey: 7,
        },
        Monkey {
            items: vec![76, 58, 57, 55, 67, 77, 54, 99],
            operation: |old| old + 6,
            test: |val| (val % 3) == 0,
            true_monkey: 2,
            false_monkey: 1,
        },
    ]
}

fn day11_helper(part_num: u8, num_rounds: usize, worry_helper: fn(i64) -> i64) {
    let mut monkeys = get_monkeys();
    let mut monkey_counts: Vec<i64> = vec![0, 0, 0, 0, 0, 0, 0, 0];

    for _ in 0..num_rounds {
        for i in 0..monkeys.len() {
            let monkey = &mut monkeys[i];

            // Avoid unsafe operation by creating a temporary vector
            let mut new_homes = vec![];
            for j in 0..monkey.items.len() {
                monkey_counts[i] += 1;
                let item = worry_helper((monkey.operation)(monkey.items[j]));
                if (monkey.test)(item) {
                    new_homes.push((monkey.true_monkey, item));
                } else {
                    new_homes.push((monkey.false_monkey, item));
                }
            }
            monkey.items.clear();
            for j in 0..new_homes.len() {
                let monkey = &mut monkeys[new_homes[j].0];
                monkey.items.push(new_homes[j].1);
            }
        }
    }
    monkey_counts.sort();
    println!(
        "The answer to part {} is: {}",
        part_num,
        monkey_counts[monkey_counts.len() - 1] * monkey_counts[monkey_counts.len() - 2]
    );
}

fn day11() {
    day11_helper(1, 20, |num| num / 3);
    day11_helper(2, 10_000, |num| num % (2 * 3 * 5 * 7 * 11 * 13 * 17 * 19));
}

fn char_to_i32(c: char) -> i32 {
    let x: u32 = c.into();
    i32::try_from(x).ok().unwrap()
}

fn day12() {
    let mut terrain: Vec<Vec<i32>> = vec![];
    let mut start_pos: (usize, usize) = (0, 0);
    let mut end_pos: (usize, usize) = (0, 0);
    if let Ok(lines) = read_lines("./input/input12") {
        for (row_num, line) in lines.enumerate() {
            if let Ok(ip) = line {
                let tip = ip.trim_end();
                let mut row = vec![];
                for (col_num, char) in tip.chars().enumerate() {
                    match char {
                        'S' => {
                            let val = char_to_i32('a') - 0x61;
                            row.push(val);
                            start_pos = (row_num, col_num);
                        }
                        'E' => {
                            let val = char_to_i32('z') - 0x61;
                            row.push(val);
                            end_pos = (row_num, col_num);
                        }
                        x => {
                            let val = char_to_i32(x) - 0x61;
                            row.push(val);
                        }
                    }
                }
                terrain.push(row);
            }
        }
    }

    let mut ans1 = 0;
    let mut queue: VecDeque<((usize, usize), u32)> = VecDeque::from([(end_pos, 0)]);
    let mut seen: HashSet<_> = HashSet::from([end_pos]);
    let mut min_dist: u32 = MAX;
    while !queue.is_empty() {
        let ((r, c), len) = queue.pop_front().unwrap();
        if terrain[r][c] == 0 {
            min_dist = if len < min_dist { len } else { min_dist };
        }
        if (r == start_pos.0) && (c == start_pos.1) {
            ans1 = len;
        }
        if r > 0 {
            let next_pos = (r - 1, c);
            if !seen.contains(&next_pos) {
                if terrain[r][c] <= terrain[next_pos.0][next_pos.1] + 1 {
                    queue.push_back((next_pos, len + 1));
                    seen.insert(next_pos);
                }
            }
        }
        if r + 1 < terrain.len() {
            let next_pos = (r + 1, c);
            if !seen.contains(&next_pos) {
                if terrain[r][c] <= terrain[next_pos.0][next_pos.1] + 1 {
                    queue.push_back((next_pos, len + 1));
                    seen.insert(next_pos);
                }
            }
        }
        if c > 0 {
            let next_pos = (r, c - 1);
            if !seen.contains(&next_pos) {
                if terrain[r][c] <= terrain[next_pos.0][next_pos.1] + 1 {
                    queue.push_back((next_pos, len + 1));
                    seen.insert(next_pos);
                }
            }
        }
        if c + 1 < terrain[0].len() {
            let next_pos = (r, c + 1);
            if !seen.contains(&next_pos) {
                if terrain[r][c] <= terrain[next_pos.0][next_pos.1] + 1 {
                    queue.push_back((next_pos, len + 1));
                    seen.insert(next_pos);
                }
            }

        }
    }
    println!("The answer to part 1 is: {}", ans1);
    println!("The answer to part 2 is: {}", min_dist);
}

fn cmp13(left: &Value, right: &Value) -> Ordering {
    match left {
        Value::Array(x) => {
            match right {
                Value::Array(y) => {
                    x.into_iter().zip(y.into_iter()).fold(
                        Ordering::Equal,
                        |acc, (xx, yy)| acc.then(cmp13(xx, yy))
                    ).then(x.len().cmp(&y.len()))
                },
                Value::Number(y) => {
                    let foo: Vec<Value> = vec![Value::Number(y.clone())];
                    x.into_iter().zip(foo.into_iter()).fold(
                        Ordering::Equal,
                        |acc, (xx, yy)| acc.then(cmp13(xx, &yy))
                    ).then(x.len().cmp(&1))
                },
                _ => panic!("No on right"),
            }
        },
        Value::Number(x) => {
            match right {
                Value::Array(y) => {
                    let foo: Vec<Value> = vec![Value::Number(x.clone())];
                    foo.into_iter().zip(y.into_iter()).fold(
                        Ordering::Equal,
                        |acc, (xx, yy)| acc.then(cmp13(&xx, yy))
                    ).then(1.cmp(&y.len()))
                },
                Value::Number(y) => {
                    match (x.as_i64(), y.as_i64()) {
                        (Some(xx), Some(yy)) => xx.cmp(&yy),
                        _ => panic!("Can't happen"),
                    }
                },
                _ => panic!("No on right"),
            }
        },
        _ => panic!("No on left"),
    }
}

fn day13() {
    let mut total = 0;
    let mut all_packets: Vec<Value> = vec![
        serde_json::from_str("[[2]]").ok().unwrap(),
        serde_json::from_str("[[6]]").ok().unwrap(),
    ];
    if let Ok(lines) = read_lines("./input/input13") {
        let mut liter = lines.into_iter();
        let mut idx = 1;
        loop {
            let line1 = match liter.next() {
                Some(Ok(x)) => x,
                _ => { break; }
            };
            let line2 = match liter.next() {
                Some(Ok(x)) => x,
                _ => { break; }
            };
            liter.next();  // Skip a blank line
            let left: Value = serde_json::from_str(&line1).ok().unwrap();
            let right: Value = serde_json::from_str(&line2).ok().unwrap();

            if cmp13(&left, &right) <= Ordering::Equal {
                total += idx;
            }
            idx += 1;

            all_packets.push(left);
            all_packets.push(right);
        }
    }
    println!("The answer to part 1 is: {}", total);
    all_packets.sort_by(cmp13);
    let l: Value = serde_json::from_str("[[2]]").ok().unwrap();
    let r: Value = serde_json::from_str("[[6]]").ok().unwrap();
    let mut li = 0;
    let mut ri = 0;
    for i in 0..all_packets.len() {
        if cmp13(&all_packets[i], &l) == Ordering::Equal {
            li = i;
        }
        if cmp13(&all_packets[i], &r) == Ordering::Equal {
            ri = i;
        }
    }
    println!("The answer to part 2 is: {}", (li + 1) * (ri + 1));
}

fn day14() {
    let mut coords: Vec<Vec<(usize, usize)>> = vec![];
    if let Ok(lines) = read_lines("./input/input14") {
        for line in lines {
            if let Ok(ip) = line {
                let tip = ip.trim_end();
                let mut row = vec![];
                for coord_pair in tip.split(" -> ") {
                    let foo: (&str, &str) = coord_pair.split(",").collect_tuple().unwrap();
                    let x: usize = foo.0.parse().unwrap();
                    let y: usize = foo.1.parse().unwrap();
                    row.push((x, y));
                }
                coords.push(row);
            }
        }
    }

    // let min_x = coords.iter().flatten().map(|x| x.0).min().unwrap();
    // let max_x = coords.iter().flatten().map(|x| x.0).max().unwrap();
    // let min_y = coords.iter().flatten().map(|x| x.1).min().unwrap();
    let max_y = coords.iter().flatten().map(|x| x.1).max().unwrap();

    // Setup the canvas
    let mut canvas: Vec<Vec<u8>> = (0..(max_y + 3)).map(|_| (0..1001).map(|_| 0).collect_vec()).collect_vec();
    for l in coords.iter() {
        for (left, right) in l.iter().tuple_windows() {
            let (left_x, left_y) = *left;
            let (right_x, right_y) = *right;
            if left_x == right_x {
                let lo_y = std::cmp::min(left_y, right_y);
                let hi_y = std::cmp::max(left_y, right_y);
                for j in lo_y..(hi_y + 1) {
                    canvas[j][left_x] = 1;
                }
            } else {
                let lo_x = std::cmp::min(left_x, right_x);
                let hi_x = std::cmp::max(left_x, right_x);
                for j in lo_x..(hi_x + 1) {
                    canvas[left_y][j] = 1;
                }
            }
        }
    }
    for j in 0..1001 {
        canvas[max_y + 2][j] = 1;
    }

    let mut count = 0;
    let mut part_one_answered = false;
    loop {
        let mut sand_x = 500;
        let mut sand_y = 0;
        count += 1;
        loop {
            if sand_y == max_y {
                if !part_one_answered {
                    println!("The answer to part 1 is: {}", count - 1);
                    part_one_answered = true;
                }
            }
            if canvas[sand_y + 1][sand_x] == 0 {
                sand_y += 1;
            } else if canvas[sand_y + 1][sand_x - 1] == 0 {
                sand_y += 1;
                sand_x -= 1;
            } else if canvas[sand_y + 1][sand_x + 1] == 0 {
                sand_y += 1;
                sand_x += 1;
            } else {
                canvas[sand_y][sand_x] = 2;
                if (sand_x == 500) && (sand_y == 0) {
                    println!("The answer to part 2 is: {}", count);
                    return;
                } else {
                    break;
                }
            }
        }
    }
}

fn get_numbers(s: &str) -> (i64, i64, i64, i64) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\d+").unwrap();
    }
    RE.find_iter(s).filter_map(|d| d.as_str().parse().ok()).collect_tuple().unwrap()
}

fn day15() {
    let mut coords: Vec<_> = vec![];
    if let Ok(lines) = read_lines("./input/input15") {
        for line in lines {
            if let Ok(ip) = line {
                coords.push(get_numbers(ip.trim_end()));
            }
        }
    }
    let num_coords = coords.len();
    // let part_one_row = 10;
    let part_one_row = 2000000;

    // What intervals are covered? Inclusive on both sides
    let mut intervals: Vec<(i64, i64)> = vec![];
    for i in 0..num_coords {
        let (sx, sy, bx, by) = coords[i];
        let dist = (sx - bx).abs() + (sy - by).abs();
        let ydist = (part_one_row - sy).abs();
        let max_x_dist = dist - ydist;
        if max_x_dist >= 0 {
            intervals.push((sx - max_x_dist, sx + max_x_dist));
        }
    }

    // Now we take the union
    intervals.sort();
    let mut new_intervals = vec![];
    let mut cur_left = intervals[0].0;
    let mut cur_right = intervals[0].1;
    for i in 1..intervals.len() {
        let (left, right) = intervals[i];
        if left <= cur_right {
            // cur_left <= left because we sorted
            // So expand to the right if it's possible
            cur_right = std::cmp::max(right, cur_right);
        } else {
            // We're starting a new interval
            new_intervals.push((cur_left, cur_right));
            cur_left = left;
            cur_right = right;
        }
    }
    // Push the last interval
    new_intervals.push((cur_left, cur_right));

    let ans_with_beacons: i64 = new_intervals.iter().map(|(l, r)| r - l + 1).sum();

    // Need to subtract out beacons in these intervals
    let mut beacons: Vec<i64> = vec![];
    for i in 0..num_coords {
        if coords[i].3 == part_one_row {
            beacons.push(coords[i].2);
        }
    }

    let mut blah: HashSet<i64> = HashSet::new();
    let num_intervals = new_intervals.len();
    for i in 0..beacons.len() {
        let bx = beacons[i];
        for j in 0..num_intervals {
            if (new_intervals[j].0 <= bx) && (bx <= new_intervals[j].1) {
                blah.insert(bx);
            }
        }
    }
    let foo: i64 = i64::try_from(blah.len()).ok().unwrap();

    println!("The answer to part 1 is: {}", ans_with_beacons - foo);

    //////////////// SHOULD LUMP INTO A FUNCTION //////////////

    for row in 0..4_000_001 {
        // What intervals are covered? Inclusive on both sides
        let mut intervals: Vec<(i64, i64)> = vec![];
        for i in 0..num_coords {
            let (sx, sy, bx, by) = coords[i];
            let dist = (sx - bx).abs() + (sy - by).abs();
            let ydist = (row - sy).abs();
            let max_x_dist = dist - ydist;
            if max_x_dist >= 0 {
                intervals.push((sx - max_x_dist, sx + max_x_dist));
            }
        }

        // Now we take the union
        intervals.sort();
        let mut new_intervals = vec![];
        let mut cur_left = intervals[0].0;
        let mut cur_right = intervals[0].1;
        for i in 1..intervals.len() {
            let (left, right) = intervals[i];
            if left <= cur_right {
                // cur_left <= left because we sorted
                // So expand to the right if it's possible
                cur_right = std::cmp::max(right, cur_right);
            } else {
                // We're starting a new interval
                new_intervals.push((cur_left, cur_right));
                cur_left = left;
                cur_right = right;
            }
        }
        // Push the last interval
        new_intervals.push((cur_left, cur_right));
        let mut min_in = new_intervals.len();
        let mut max_in = new_intervals.len();
        for i in 0..new_intervals.len() {
            if (new_intervals[i].0 <= 0) && (0 <= new_intervals[i].1) {
                min_in = i;
            }
            if (new_intervals[i].0 <= 4_000_000) && (4_000_000 <= new_intervals[i].1) {
                max_in = i;
            }
        }
        if min_in != max_in {
            // Should actually check that there are two intervals; but visual inspection
            // indicates that there are only two in this row
            println!("The answer to part 2 is: {}", (new_intervals[0].1 + 1) * 4_000_000 + row);
        }
    }
}

fn main() {
    day01();
    day02();
    day03();
    day04();
    day05();
    day06();
    day07();
    day08();
    day09();
    day10();
    day11();
    day12();
    day13();
    day14();
    day15();
}
