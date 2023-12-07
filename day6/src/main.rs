use std::fs;

fn part1(times: &Vec<usize>, distances: &Vec<usize>) -> usize {
    let mut possible_races: Vec<usize> = Vec::new();
    for (i, target_time) in times.iter().enumerate() {
        let target_distance = distances[i];
        possible_races.push(part2(*target_time, target_distance));
    }
    possible_races.into_iter().reduce(|a, b| a * b).unwrap()
}

fn part2(target_time: usize, target_distance: usize) -> usize {
    let mut possible = 0;
    for speed in 0..=target_time {
        let time = target_time - speed;
        let distance = time * speed;
        if distance > target_distance {
            possible += 1;
        } else if possible > 0 {
            // All possibilities already exhausted
            break;
        }
    }
    possible
}

fn parse_input(line: &str) -> Vec<usize> {
    line.split_whitespace()
        .filter_map(|part| part.parse().ok())
        .collect()
}

fn concat_input(input: &Vec<usize>) -> usize {
    input
        .iter()
        .map(|n| n.to_string())
        .collect::<String>()
        .parse()
        .unwrap()
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let (in_times, in_distances) = input.split_once('\n').unwrap();
    let times: Vec<usize> = parse_input(in_times);
    let distances: Vec<usize> = parse_input(in_distances);
    let part1_result = part1(&times, &distances);
    println!("Result for part 1: {part1_result}");
    let time_pt2: usize = concat_input(&times);
    let distance_pt2: usize = concat_input(&distances);
    let part2_result = part2(time_pt2, distance_pt2);
    println!("Result for part 2: {:?}", part2_result);
}
