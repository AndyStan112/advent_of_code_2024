use std::io;
use std::io::BufRead;

fn read_multiline_input() -> String {
    let stdin = io::stdin();
    let mut all_input = String::new();

    for line in stdin.lock().lines() {
        let line = line.expect("Failed to read line");
        all_input.push_str(&line);
        all_input.push(' ');
    }

    all_input.trim().to_string()
}
fn sum_mult(str: &str) -> i32 {
    str.split("mul(")
        .filter_map(|token| {
            let token = extract_before_paranthesis(&token)?;
            let numbers = extract_numbers(token);

            if numbers.len() != 2 || numbers.iter().any(|num| num.is_none()) {
                return Some(0);
            }

            Some(numbers.iter().map(|num| num.unwrap()).product())
        })
        .sum()
}

fn extract_numbers(token: &str) -> Vec<Option<i32>> {
    let numbers: Vec<Option<i32>> = token
        .split(',')
        .map(|num| num.parse::<i32>().ok())
        .collect();
    numbers
}

fn extract_before_paranthesis(token: &str) -> Option<&str> {
    match token.split_once(')') {
        Some((left, _)) if !left.is_empty() => Some(left),
        _ => None,
    }
}
pub fn part1() {
    let input_string = read_multiline_input();

    dbg!(sum_mult(&input_string));
}
fn extract_command(token: &str) -> Option<&str> {
    match token.split_once("do()") {
        Some((_, command)) if !command.is_empty() => Some(command),
        _ => None,
    }
}
fn com_sum(str: &str) -> i32 {
    str.split("don't()")
        .filter_map(|mut token| {
            let command = extract_command(&token)?;
            Some(sum_mult(&command))
        })
        .sum()
}

pub fn part2() {
    let input_string = "do()".to_owned() + &read_multiline_input();
    dbg!(com_sum(&input_string));
}
