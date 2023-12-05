mod map;
use map::{Map, MapRange};
use std::cmp;
use std::fs::File;
use std::io::{self, BufRead};

fn part1() -> usize {
    let file = File::open("./input.txt").unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut seeds: Vec<usize> = Vec::new();
    let mut maps: Vec<Map> = Vec::new();

    {
        let mut active_map: Map = Default::default();

        for rl in lines {
            let line = rl.unwrap();
            if line.len() < 1 {
                continue;
            }

            if line.starts_with("seeds:") {
                let (_, lseeds) = line.split_once(':').unwrap();
                for seed in lseeds.trim().split(' ') {
                    seeds.push(seed.parse().unwrap());
                }
            }

            if line.contains(':') {
                if active_map.ranges.len() > 0 {
                    maps.push(active_map);
                    active_map = Default::default();
                    active_map.name = line;
                }
                continue;
            }

            active_map.ranges.push(MapRange::from_str(&line));
        }

        if active_map.ranges.len() > 0 {
            maps.push(active_map);
        }
    }

    for map in maps {
        println!("{}", map.name);
        for seed in seeds.iter_mut() {
            let initial_val = *seed;
            let new_val = map.transform_single(initial_val);
            *seed = new_val;
            println!("{} -> {}", initial_val, new_val);
        }
    }

    let min = seeds.iter().reduce(|a, b| cmp::min(a, b)).unwrap();

    *min
}

fn part2() -> usize {
    let file = File::open("./input.txt").unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut ranges: Vec<Range> = Vec::new();
    let mut maps: Vec<Map> = Vec::new();

    {
        let mut active_map: Map = Default::default();
        for rl in lines {
            let line = rl.unwrap();
            if line.len() < 1 {
                continue;
            }

            if line.starts_with("seeds:") {
                let (_, lseeds) = line.split_once(':').unwrap();
                let mut ranges_str = lseeds.trim().split(' ').peekable();
                while ranges_str.peek().is_some() {
                    let mut range_str = ranges_str.by_ref().take(2);
                    let start: usize = range_str.next().unwrap().parse().unwrap();
                    let length: usize = range_str.next().unwrap().parse().unwrap();
                    ranges.push(Range { start, length });
                }
                continue;
            }

            if line.contains(':') {
                if active_map.ranges.len() > 0 {
                    maps.push(active_map);
                    active_map = Default::default();
                    active_map.name = line;
                }
                continue;
            }

            active_map.ranges.push(MapRange::from_str(&line));
        }

        if active_map.ranges.len() > 0 {
            maps.push(active_map);
        }
    }

    // println!("{:?}", ranges);
    println!("{:?}", ranges);
    for map in maps {
        println!("{}", map.name);
        ranges = map.transform_ranges(&ranges);
        println!("{:?}", ranges);
    }

    let mut min = usize::MAX;
    for r in ranges {
        min = cmp::min(min, r.start);
    }

    min
}

#[derive(Debug, Clone)]
pub struct Range {
    pub start: usize,
    pub length: usize,
}

fn main() {
    let part1_result = part1();
    println!("Result for part 1: {part1_result}");
    let part2_result = part2();
    println!("Result for part 2: {part2_result}");
}
