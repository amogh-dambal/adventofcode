use std::{
    cmp::{max, min},
    collections::BinaryHeap,
    io::stdin,
    iter::zip,
};

fn main() {
    part1();
}

fn part1() {
    let mut lefts: Vec<u32> = Vec::new();
    let mut rights: Vec<u32> = Vec::new();

    stdin()
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();

            let left = parts[0].parse::<u32>().unwrap();
            let right = parts[1].parse::<u32>().unwrap();

            (left, right)
        })
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
