use std::fs::File;
use std::io::{self, BufRead};

fn part1() -> usize {
    let file = File::open("./input.txt").unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut sum = 0;
    for line in lines {
        let card = Card::from_str(&line.unwrap());
        let mut score: usize = 0;
        for num in card.numbers {
            if card.winning.iter().any(|n| *n == num) {
                if score > 0 {
                    score *= 2;
                } else {
                    score += 1;
                }
            }
        }
        sum += score;
    }
    sum
}

fn part2() -> usize {
    let file = File::open("./input.txt").unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut pile: Vec<Card> = Vec::new();
    for line in lines {
        let card = Card::from_str(&line.unwrap());
        pile.push(card);
    }

    let mut i = 0;
    loop {
        if i >= pile.len() {
            break;
        };

        let card = &pile[i].clone();
        let mut score: usize = 0;
        for num in &card.numbers {
            if card.winning.iter().any(|n| *n == *num) {
                score += 1;
            }
        }

        for offset in 1..=score {
            let to_insert = pile
                .iter()
                .find(|other_card| other_card.id == card.id + offset);
            if let Some(new_card) = to_insert {
                pile.push(new_card.clone());
            } else {
                println!("Card not found {}", card.id + offset)
            }
        }

        i += 1;
    }

    pile.len()
}

#[derive(Debug, Clone)]
struct Card {
    id: usize,
    winning: Vec<usize>,
    numbers: Vec<usize>,
}

impl Card {
    fn from_str(input: &str) -> Self {
        let card_id: usize = input[4..8].trim().parse().unwrap();
        let (winning, numbers) = input[9..].split_once('|').unwrap();
        Self {
            id: card_id,
            winning: winning
                .split(' ')
                .filter_map(|n| {
                    if n.is_empty() {
                        None
                    } else {
                        Some(n.parse().unwrap())
                    }
                })
                .collect(),
            numbers: numbers
                .split(' ')
                .filter_map(|n| {
                    if n.is_empty() {
                        None
                    } else {
                        Some(n.parse().unwrap())
                    }
                })
                .collect(),
        }
    }
}

fn main() {
    let part1_result = part1();
    println!("Result for part 1: {part1_result}");
    let part2_result = part2();
    println!("Result for part 2: {part2_result}");
}
