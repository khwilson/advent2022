use core::num;
use std::cmp::max;
use std::collections::{HashSet, HashMap, VecDeque};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec;
use itertools::Itertools;

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
}
