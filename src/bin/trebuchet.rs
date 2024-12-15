use std::io::stdin;

use regex::Regex;

fn main() {
    // part_1();
    part_2();
}

#[allow(unused)]
fn part_1() {
    let calibration_sum: u32 = stdin()
        .lines()
        .map(|line| {
            match line {
                Ok(val) => get_calibration_number(&val),
                Err(err) => panic!("{}", err)
            }
        })
        .sum();

    println!("{}", calibration_sum);
}

fn get_calibration_number(s: &String) -> u32 {
    let mut first = 0;
    let mut last = 0;

    for ch in s.chars().rev() {
        if let Some(val) = ch.to_digit(10) {
            last = val;
            break;
        }
    }
    for ch in s.chars() {
        if let Some(val) = ch.to_digit(10) {
            first = val;
            break;
        }
    }


    return first * 10 + last;
}

const TREBUCHET_DIGIT_MATCHER: &str = r"^.*(one|two|three|four|five|six|seven|eight|nine|[\d]).*(one|two|three|four|five|six|seven|eight|nine|[\d])?.*$";

fn part_2() {
    let mut calibration_sum = 0;
    // Regex compilation is expensive, so we pull it out 
    // of the loop.
    let re = Regex::new(TREBUCHET_DIGIT_MATCHER).unwrap();
    for line in stdin().lines() {
        match line {
            Ok(val) => {
                println!("line: {}", val);
                calibration_sum += get_advanced_calibration_number(&re, &val);
            },
            Err(err) => panic!("error reading line: {}", err)
        }
    }
    println!("{}", calibration_sum);
}

fn get_advanced_calibration_number(re: &Regex, s: &String) -> u32 {
    let first;
    let last;
    let captures = re.captures(s).expect("Captures panicked!");


    if captures.len() == 3 {
        let val = captures.get(0).expect("No capture 0").as_str();
        println!("val: {}", val);
        first = get_digit_val(val);
        last = first;
    } 
    else {
        let fval = captures.get(0).expect("No capture 0").as_str();
        let lval = captures.get(captures.len()-1).expect("No capture 0").as_str();
        println!("fval: {}\tlval: {}", fval, lval);
        first = get_digit_val(fval);
        last = get_digit_val(lval);
    }
    
    // println!("parsed {}, {} from line", first, last);

    return first * 10 + last;
}

fn get_digit_val(s: &str) -> u32 {
    match s {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => {
            match s.parse::<u32>() {
                Ok(val) => val,
                Err(err) => panic!("error calling s.parse::<u32>(): {}", err)
            }
        }
    }
}