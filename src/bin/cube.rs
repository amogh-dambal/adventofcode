use core::panic;
use std::io::stdin;

const RED: u32 = 12;
const GREEN: u32 = 13;
const BLUE: u32 = 14;

fn main() {
    part_2();
}

fn part_2() {
    let sum_power_sets: u32 = stdin()
        .lines()
        .map(|line| match line {
            Ok(s) => return get_power_set(s),
            Err(err) => panic!("unable to read from STDIN: {}", err),
        })
        .sum();

    println!("{}", sum_power_sets);
}

fn get_power_set(s: String) -> u32 {
    let parts: Vec<&str> = s.split(":").collect();

    let mut blue: u32 = 1;
    let mut green: u32 = 1;
    let mut red: u32 = 1;

    parts[1]
        .split(|c| {
            return c == ',' || c == ';';
        })
        .for_each(|draw| {
            let game_info: Vec<&str> = draw.split_whitespace().collect();
            let cubes;
            match game_info[0].parse::<u32>() {
                Ok(val) => cubes = val,
                Err(err) => panic!("error parsing value: {}", err),
            }

            match game_info[1] {
                "red" => red = red.max(cubes),
                "green" => green = green.max(cubes),
                "blue" => blue = blue.max(cubes),
                _ => panic!("invalid color"),
            };
        });

    // println!("{} red, {} green, {} blue", red, green, blue);
    return red * green * blue;
}

// Part 1
// https://adventofcode.com/2023/day/2#part1
#[allow(unused)]
fn part_1() {
    let sum_game_ids: u32 = stdin()
        .lines()
        .map(|line| match line {
            Ok(s) => {
                return get_game_id(s);
            }
            Err(err) => {
                panic!("Error while reading lines: {}", err);
            }
        })
        .sum();

    println!("{}", sum_game_ids);
}

fn get_game_id(s: String) -> u32 {
    let parts: Vec<&str> = s.split(":").collect();

    // Get the game data
    let all_possible = parts[1]
        .split(|c| {
            return c == ',' || c == ';';
        })
        .all(|cube_draw| {
            return is_cube_draw_possible(&cube_draw);
        });

    if !all_possible {
        return 0;
    } else {
        let game_id_part: Vec<&str> = parts[0].split(" ").collect();
        match game_id_part[game_id_part.len() - 1].parse::<u32>() {
            Ok(val) => return val,
            Err(err) => panic!("error parsing Game ID: {err}"),
        }
    }
}

fn is_cube_draw_possible(draw: &str) -> bool {
    let game_info: Vec<&str> = draw.split_whitespace().collect();
    let cubes;
    match game_info[0].parse::<u32>() {
        Ok(val) => cubes = val,
        Err(err) => panic!("error parsing value: {}", err),
    }

    match game_info[1] {
        "red" => return cubes <= RED,
        "green" => return cubes <= GREEN,
        "blue" => return cubes <= BLUE,
        _ => panic!("invalid color"),
    }
}

