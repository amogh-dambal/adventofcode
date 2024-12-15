use std::io::stdin;

fn main() {
    part1();
}

fn read_line(line: &str) -> Vec<u32> {
    line.split_whitespace()
        .map(|part| part.parse::<u32>().expect("Unable to parse line!"))
        .collect()
}

fn part1() {
    let safe_reports: usize = stdin()
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| read_line(&line))
        .filter(|levels| get_safety(&levels))
        .count();
    println!("{}", safe_reports);
}

fn get_safety(levels: &Vec<u32>) -> bool {
    let mut diff: i32 = 0;
    let diff_check = levels
        .windows(2)
        .map(|window| {
            let left = window[0];
            let right = window[1];

            if left > right {
                diff -= 1;
            } else if left < right {
                diff += 1
            }

            (1..=3).contains(&left.abs_diff(right))
        })
        .all(|x| x);

    let is_safe: bool = diff.abs() == (levels.len() - 1).try_into().unwrap() && diff_check;
    is_safe
}
