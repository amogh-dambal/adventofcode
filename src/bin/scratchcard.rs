use std::{collections::{HashMap, HashSet}, io::stdin, ops::Range};

fn main() {
    // part1();
    part2();
}

fn part2() {
    let mut card_tracker: HashMap<usize, usize> = HashMap::new();

    stdin()
        .lines()
        .map(|line| {
            match line {
                Ok(card) => return get_matches(card),
                Err(err) => panic!("unable to read line: {}", err),
            }
        })
        .enumerate()
        .for_each(|(i, val)| {
            apply_card_copies(&mut card_tracker, i, val as usize);
        });

    // println!("card tracker: {:#?}", card_tracker);
    let copies: usize = card_tracker.values().sum();
    println!("{}", copies);
}

fn apply_card_copies(cards: &mut HashMap<usize, usize>, id: usize, matches: usize) {
    let new_copies = *cards.get(&id).unwrap_or_else(|| &0) + 1;
    cards.insert(id, new_copies);
    
    let start: usize = id+1;
    let end: usize = start + matches;
    for c in (Range{start: start, end: end}) {
        let match_copies = *cards.get(&c).unwrap_or_else(|| &0);
        cards.insert(c, match_copies + new_copies);
    }
}


fn get_matches(card: String) -> u32 {
    // We don't care about the card ID, so just drop everything until we get
    // the winning numbers and our numbers.
    let card_numbers = card
        .chars()
        .skip_while(|c| {
            return *c != ':';
        })
        // We want to start our new iterator cursor at the beginning of all the numbers.
        // We can do a .skip(1) since we'll be splitting on whitespace later as well.
        .skip(2)
        .collect::<String>();

    // Create an iterator over all the numbers, plus the '|' delimiter
    // between the winning numbers and our numbers.
    // By using '.by_ref()' cleverly, we can use this singular iterator
    // for both traversals which means we only go through the input line once.
    let mut numbers = card_numbers.split_whitespace();
    let winning_numbers: HashSet<u32> = HashSet::from_iter(
        numbers.by_ref()
            .take_while(|s| {
                return *s != "|";
            })
            .filter_map(|s| {
                return s.parse::<u32>().ok();
            })
    );

    let matches: usize = numbers
        .filter_map(|s| {
            return s.parse::<u32>().ok();
        })
        .filter(|n| {
            return winning_numbers.contains(n);
        })
        .count();

    return matches as u32;
}

fn part1() {
    let points: u32 = stdin()
        .lines()
        .map(|line| {
            match line {
                Ok(card) => return get_points(card),
                Err(err) => panic!("encountered error reading line: {}", err),
            }
        })
        .sum();

    println!("{}", points);
}

fn get_points(card: String) -> u32 {
    // We don't care about the card ID, so just drop everything until we get
    // the winning numbers and our numbers.
    let card_numbers = card
        .chars()
        .skip_while(|c| {
            return *c != ':';
        })
        // We want to start our new iterator cursor at the beginning of all the numbers.
        // We can do a .skip(1) since we'll be splitting on whitespace later as well.
        .skip(2)
        .collect::<String>();

    // Create an iterator over all the numbers, plus the '|' delimiter
    // between the winning numbers and our numbers.
    // By using '.by_ref()' cleverly, we can use this singular iterator
    // for both traversals which means we only go through the input line once.
    let mut numbers = card_numbers.split_whitespace();
    let winning_numbers: HashSet<u32> = HashSet::from_iter(
        numbers.by_ref()
            .take_while(|s| {
                return *s != "|";
            })
            .filter_map(|s| {
                return s.parse::<u32>().ok();
            })
    );

    let points: u32 = numbers
        .filter_map(|s| {
            return s.parse::<u32>().ok();
        })
        .filter(|n| {
            return winning_numbers.contains(n);
        })
        .enumerate()
        .map(|(i, _)| {
            return 2_u32.pow(i as u32);
        })
        .max()
        .unwrap_or_default();
    
    return points;
}