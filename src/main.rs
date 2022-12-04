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

fn main() {
    day01();
    day02();
    day03();
}
