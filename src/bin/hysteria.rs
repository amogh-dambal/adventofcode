use std::{
    cmp::{max, min},
    collections::{BinaryHeap, HashMap},
    io::stdin,
    iter::zip,
};

fn main() {
    //    part1();
    part2();
}

fn read_line(line: &str) -> (u32, u32) {
    let parts: Vec<&str> = line.split_whitespace().collect();

    let left = parts[0].parse::<u32>().unwrap();
    let right = parts[1].parse::<u32>().unwrap();

    (left, right)
}

#[allow(unused)]
fn part1() {
    let mut lefts: Vec<u32> = Vec::new();
    let mut rights: Vec<u32> = Vec::new();

    stdin()
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| read_line(&line))
        .for_each(|(left, right)| {
            lefts.push(left);
            rights.push(right);
        });

    let left_locations = BinaryHeap::from(lefts).into_sorted_vec();
    let right_locations = BinaryHeap::from(rights).into_sorted_vec();

    let total_distance: u32 = zip(left_locations, right_locations)
        .map(|(left, right)| max(left, right) - min(left, right))
        .sum();
    println!("{}", total_distance);
}

#[allow(unused)]
fn part2() {
    let mut location_ids: Vec<u32> = Vec::new();
    let mut similarity_multiplier: HashMap<u32, u32> = HashMap::new();

    stdin()
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| read_line(&line))
        .for_each(|(left, right)| {
            location_ids.push(left);
            similarity_multiplier
                .entry(right)
                .and_modify(|val| *val += 1)
                .or_insert(1);
        });

    let similarity_score: u32 = location_ids
        .iter()
        .map(|location_id| match similarity_multiplier.get(location_id) {
            Some(val) => location_id * val,
            None => 0,
        })
        .sum();
    println!("{}", similarity_score);
}
