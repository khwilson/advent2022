use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
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

fn main() {
    day01();
    day02();
    day03();
    day04();
    day05();
}
