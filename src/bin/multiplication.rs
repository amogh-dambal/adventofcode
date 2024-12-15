/// Advent of Code 2024
/// Day 3
use std::io::stdin;

use regex::Regex;

fn main() {
    part2();
}

#[allow(unused)]
fn part1() {
    let re = Regex::new(r"mul\((?<first>[0-9]{1,3}),(?<second>[0-9]{1,3})\)").unwrap();

    let result = stdin()
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            re.captures_iter(&line)
            .map(|c| c.extract())
            .map(|(_, [first, second])| (first.parse::<i32>().unwrap(), second.parse::<i32>().unwrap()))
            .map(|(x, y)| x * y)
            .sum::<i32>()
        })
        .sum::<i32>();
}

#[allow(unused)]
fn part2() {
    let re = Regex::new(r"mul\((?<first>[0-9]{1,3}),(?<second>[0-9]{1,3})\)|don't\(\)|do\(\)").unwrap();

    let mut enabled: bool = true;
    let result: u128 = stdin()
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            let mut total: u128 = 0;
            for cap in re.captures_iter(&line) {
                if let Some(full) = cap.get(0) {
                    match full.as_str() {
                        "don't()" => {
                            enabled = false;
                        },
                        "do()" => {
                            enabled = true;
                        },
                        _ => {
                            if enabled {
                                let x: u128 = cap.name("first").unwrap().as_str().parse::<u128>().unwrap();
                                let y: u128 = cap.name("second").unwrap().as_str().parse::<u128>().unwrap();
                                total += x * y
                            }
                        }
                    }
                }
            }
            total
        })
        .sum();

}