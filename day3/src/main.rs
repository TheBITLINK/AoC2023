use std::fs::File;
use std::io::{self, BufRead};

fn part1(parts: &Vec<Segment>, numbers: &Vec<Segment>) -> u64 {
    // Iterate over numbers and find adjacent parts
    let mut sum = 0;
    for s in numbers {
        if let SegmentKind::Number(num) = s.kind {
            let x1 = s.pos.x.saturating_sub(1);
            let x2 = s.pos.x + s.width + 1;
            let y1 = s.pos.y.saturating_sub(1);
            let y2 = s.pos.y + 1;
            for y in y1..=y2 {
                let symb = parts
                    .iter()
                    .find(|s| s.pos.y == y && s.pos.x + s.width > x1 && s.pos.x + s.width <= x2);
                if let Some(_) = symb {
                    sum += num;
                }
            }
        }
    }

    sum
}

fn part2(parts: &Vec<Segment>, numbers: &Vec<Segment>) -> u64 {
    let mut sum = 0;
    for s in parts {
        // Skip non-gear parts
        if let SegmentKind::Part = s.kind {
            continue;
        }
        let x1 = s.pos.x.saturating_sub(1);
        let x2 = s.pos.x + 1;
        let y1 = s.pos.y.saturating_sub(1);
        let y2 = s.pos.y + 1;
        let adjacent_numbers: Vec<u64> = numbers
            .iter()
            .filter_map(|s| {
                if s.pos.y >= y1
                    && s.pos.y <= y2
                    && ((s.pos.x >= x1 && s.pos.x <= x2)
                        || (s.pos.x + s.width > x1 && s.pos.x + s.width <= x2))
                {
                    if let SegmentKind::Number(num) = s.kind {
                        return Some(num);
                    }
                }
                None
            })
            .collect();
        if adjacent_numbers.len() == 2 {
            sum += adjacent_numbers[0] * adjacent_numbers[1]
        }
    }
    sum
}

#[derive(Default, Debug, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy)]
enum SegmentKind {
    Number(u64),
    Part,
    Gear,
}

#[derive(Debug, Clone, Copy)]
struct Segment {
    kind: SegmentKind,
    pos: Point,
    width: usize,
}

fn main() {
    let file = File::open("./input.txt").unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut parts: Vec<Segment> = Vec::new();
    let mut numbers: Vec<Segment> = Vec::new();
    let mut cursor: Option<Segment> = None;

    // First pass: Identify Positions
    for (y, line) in lines.enumerate() {
        let row = line.unwrap();
        for (x, c) in row.chars().enumerate() {
            match c {
                // Dots
                '.' => {
                    if let Some(cur) = cursor {
                        match cur.kind {
                            SegmentKind::Number(_) => numbers.push(cur),
                            SegmentKind::Part => parts.push(cur),
                            SegmentKind::Gear => parts.push(cur),
                        }
                        cursor = None;
                    }
                    continue;
                }
                // Numbers
                '0'..='9' => {
                    let digit: u64 = c.to_string().parse().unwrap_or(0);
                    if let Some(ref mut cur) = cursor {
                        match cur.kind {
                            SegmentKind::Number(num) => {
                                cur.width += 1;
                                cur.kind = SegmentKind::Number(num * 10 + digit);
                                continue;
                            }
                            SegmentKind::Part => parts.push(*cur),
                            SegmentKind::Gear => parts.push(*cur),
                        }
                    }
                    cursor = Some(Segment {
                        kind: SegmentKind::Number(digit),
                        pos: Point { x, y },
                        width: 1,
                    })
                }
                // Assume everything else is a symbol
                _ => {
                    if let Some(ref mut cur) = cursor {
                        match cur.kind {
                            SegmentKind::Part => {
                                cur.width += 1;
                                continue;
                            }
                            SegmentKind::Gear => parts.push(*cur),
                            SegmentKind::Number(_) => numbers.push(*cur),
                        }
                    }
                    cursor = Some(Segment {
                        kind: if c == '*' {
                            SegmentKind::Gear
                        } else {
                            SegmentKind::Part
                        },
                        pos: Point { x, y },
                        width: 1,
                    })
                }
            }
        }
        if let Some(cur) = cursor {
            match cur.kind {
                SegmentKind::Number(_) => numbers.push(cur),
                SegmentKind::Part => parts.push(cur),
                SegmentKind::Gear => parts.push(cur),
            }
            cursor = None;
        }
    }

    let part1_result = part1(&parts, &numbers);
    println!("Result for part 1: {part1_result}");
    let part2_result = part2(&parts, &numbers);
    println!("Result for part 2: {part2_result}");
}
