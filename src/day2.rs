use std::io;

fn check_report(part: i8) -> Option<bool> {
    let mut input = String::new();

    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let trimmed = input.trim();
    if trimmed.is_empty() {
        return None;
    }
    let report: Vec<i32> = trimmed
        .split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();
    if report.len() <= 1 {
        return Some(true);
    }
    let increasing_condition = |(a, b): (i32, i32)| (a > b) && (a - b <= 3);
    let decreasing_condition = |(a, b): (i32, i32)| (b > a) && (b - a <= 3);
    match part {
        1 => Some(check1(&report, increasing_condition)? || check1(&report, decreasing_condition)?),
        2 => Some(check2(&report, increasing_condition)? || check2(&report, decreasing_condition)?),
        _ => todo!(),
    }
}

fn check1(report: &[i32], condition: fn((i32, i32)) -> bool) -> Option<bool> {
    Some(report.windows(2).all(|pair| condition((pair[0], pair[1]))))
}
fn check2(report: &[i32], condition: fn((i32, i32)) -> bool) -> Option<bool> {
    if check1(&report, condition)? {
        return Some(true);
    }

    for i in 0..report.len() {
        let mut new_report = report.to_vec();
        new_report.remove(i);
        if check1(&new_report, condition)? {
            return Some(true);
        }
    }

    Some(false)
}

pub fn part1() {
    let mut s = 0;

    loop {
        match check_report(1) {
            None => {
                break;
            }
            Some(true) => {
                s += 1;
            }
            _ => {}
        }
    }

    println!("{}", s);
}

pub fn part2() {
    let mut s = 0;

    loop {
        match check_report(2) {
            None => {
                break;
            }
            Some(true) => {
                s += 1;
            }
            _ => {}
        }
    }

    println!("{}", s);
}
