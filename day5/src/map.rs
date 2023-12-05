use crate::Range;
use std::cmp;

#[derive(Debug, Clone)]
pub struct MapRange {
    pub src: usize,
    pub dst: usize,
    pub len: usize,
}

impl MapRange {
    pub fn from_str(input: &str) -> Self {
        let mut values = input.split(' ');
        Self {
            dst: values.next().unwrap().parse().unwrap(),
            src: values.next().unwrap().parse().unwrap(),
            len: values.next().unwrap().parse().unwrap(),
        }
    }
}

#[derive(Default)]
pub struct Map {
    pub name: String,
    pub ranges: Vec<MapRange>,
}

impl Map {
    pub fn transform_single(&self, input: usize) -> usize {
        for r in &self.ranges {
            if input >= r.src && input < r.src + r.len {
                let offset = input - r.src;
                return r.dst + offset;
            }
        }
        input
    }

    pub fn transform_ranges(&self, input: &Vec<Range>) -> Vec<Range> {
        let mut output: Vec<Range> = Vec::new();

        // println!("{:?}", input);

        for range in input {
            let mut start = range.start;
            let end = start + range.length;
            let mut dirty = false;

            for transform_range in &self.ranges {
                let tx_start = transform_range.src;
                let tx_end = tx_start + transform_range.len;

                if tx_start > end || tx_end < start {
                    continue;
                }

                dirty = true;

                if start < tx_start {
                    output.push(Range {
                        start,
                        length: tx_start - start,
                    });
                }

                let actual_start = cmp::max(start, tx_start);
                let actual_end = cmp::min(end, tx_end);

                output.push(Range {
                    start: transform_range.dst + actual_start - tx_start,
                    length: actual_end - actual_start,
                });

                if end > tx_end {
                    start = tx_end;
                    continue;
                }
            }

            if !dirty {
                output.push(range.clone());
            }
        }

        output
    }
}
