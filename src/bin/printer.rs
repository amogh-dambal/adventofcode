/// Advent of Code 2024
/// Day 5
use std::io::stdin;

use petgraph::graph::{DiGraph, NodeIndex};

type RuleGraph = DiGraph<i32, ()>;

fn main() {
    part1();
}

fn part1() {
    let rules = read_rules();
    
    let page_num_total: u32 = stdin()
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| parse_update(&line))
        .filter(|update| is_in_order(update, &rules))
        .map(|update| update[update.len() / 2])
        .sum();

    println!("{}", page_num_total);
}

/// Read the printer queuing rules from STDIN.
/// The rule X|Y means that X must appear before Y in the update.
/// These rules can be modeled as a digraph. For example, given:
/// ```
/// X|Y
/// X|Z
/// Z|A
/// Y|B
/// Z|C
/// Y|D
/// ```
/// We can construct the following digraph (represented as adjacency lists):
/// (x -> [y, z, a, b, c, d]), (z -> [a, c]), (y -> [b, d])
/// Note that the rules are transitive - X must come before Y, which must come before B,
/// therefore X must come before B.
fn read_rules() -> RuleGraph {
    RuleGraph::from_edges(
    stdin()
        .lines()
        .take_while(|line| !line.as_ref().unwrap().is_empty())
        .filter_map(|line| line.ok())
        .map(|line| {
            let parts = line.split("|").collect::<Vec<&str>>();
            let x = parts[0].parse::<u32>().unwrap();
            let y = parts[1].parse::<u32>().unwrap();
            (x, y)
        })
        .collect::<Vec<(u32, u32)>>()
    )
}

fn parse_update(line: &str) -> Vec<u32> {
    line.split(",").map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>()
}

fn is_in_order(update: &Vec<u32>, rules: &RuleGraph) -> bool {
    update
        .windows(2)
        .map(|window| {
            rules.contains_edge(
                NodeIndex::new(window[0] as usize),
                NodeIndex::new(window[1] as usize)
        )
        })
        .all(|x| x)
}