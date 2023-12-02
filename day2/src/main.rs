use std::cmp;
use std::fs::File;
use std::io::{self, BufRead};

fn part1() -> u64 {
    let file = File::open("./input.txt").unwrap();
    let lines = io::BufReader::new(file).lines();
    // Maximum number of cubes in the bag
    let bag = Cubes {
        red: 12,
        green: 13,
        blue: 14,
    };

    let mut sum = 0;
    for line in lines {
        let game = Game::from_str(&line.unwrap());
        if game.max.red > bag.red || game.max.green > bag.green || game.max.blue > bag.blue {
            continue;
        }
        sum += game.id;
    }

    sum
}

fn part2() -> u64 {
    let file = File::open("./input.txt").unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut sum = 0;
    for line in lines {
        let game = Game::from_str(&line.unwrap());
        sum += game.max.red * game.max.green * game.max.blue;
    }

    sum
}

#[derive(Default, Debug)]
struct Cubes {
    red: u64,
    green: u64,
    blue: u64,
}

impl Cubes {
    fn from_str(input: &str) -> Self {
        let mut cubes: Self = Default::default();
        let parts = input.trim().split(',');
        for part in parts {
            let (quantity, color) = part.trim().split_once(' ').unwrap();
            match color {
                "red" => cubes.red += quantity.parse::<u64>().unwrap(),
                "green" => cubes.green += quantity.parse::<u64>().unwrap(),
                "blue" => cubes.blue += quantity.parse::<u64>().unwrap(),
                _ => continue,
            }
        }
        cubes
    }
}

#[derive(Debug)]
struct Game {
    id: u64,
    moves: Vec<Cubes>,
    max: Cubes,
}

impl Game {
    fn from_str(input: &str) -> Self {
        let (header, movelist) = input.split_once(':').unwrap();
        let (_, game_id) = header.split_once(' ').unwrap();
        let mut game = Game {
            id: game_id.parse().unwrap(),
            moves: Vec::new(),
            max: Default::default(),
        };
        let moves = movelist.split(';');
        for mov in moves {
            let cubes = Cubes::from_str(&mov);
            game.max.red = cmp::max(game.max.red, cubes.red);
            game.max.green = cmp::max(game.max.green, cubes.green);
            game.max.blue = cmp::max(game.max.blue, cubes.blue);
            game.moves.push(cubes);
        }
        game
    }
}

fn main() {
    let part1_result = part1();
    println!("Result for part 1: {part1_result}");
    let part2_result = part2();
    println!("Result for part 2: {part2_result}");
}
