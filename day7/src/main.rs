mod hand;
use hand::Hand;
use std::fs::File;
use std::io::{self, BufRead};

fn execute(is_part_2: bool) -> usize {
    let file = File::open("input.txt").unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut hands: Vec<Hand> = Vec::new();
    for line in lines {
        hands.push(Hand::from_str(&line.unwrap(), is_part_2));
    }
    let mut winnings = 0;
    hands.sort_by(|a, b| a.numeric_value.cmp(&b.numeric_value));
    for (rank, hand) in hands.iter().enumerate() {
        winnings += hand.bid * (rank + 1);
    }
    winnings
}

fn main() {
    let part1_result = execute(false);
    println!("Result for part 1: {part1_result}");
    let part2_result = execute(true);
    println!("Result for part 2: {part2_result}");
}
