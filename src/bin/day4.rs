use std::str::FromStr;

#[derive(Debug)]
struct Range {
    first_start: usize,
    first_end: usize,
    second_start: usize,
    second_end: usize,
}

impl FromStr for Range {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let two_ranges = &s.split(',').collect::<Vec<_>>()[0..=1];
        let first_range = two_ranges[0].split('-').collect::<Vec<_>>();
        let second_range = two_ranges[1].split('-').collect::<Vec<_>>();
        Ok(Self {
            first_start: first_range[0].parse().unwrap(),
            first_end: first_range[1].parse().unwrap(),
            second_start: second_range[0].parse().unwrap(),
            second_end: second_range[1].parse().unwrap(),
        })
    }
}

fn part_one(input: &str) -> usize {
    let mut inclusive_count = 0;
    for line in input.lines() {
        let range: Range = line.parse().unwrap();
        if range.first_start < range.second_start {
            if range.first_end >= range.second_end {
                inclusive_count += 1;
            }
        } else if range.first_start == range.second_start || range.first_end <= range.second_end {
            inclusive_count += 1;
        }
    }
    inclusive_count
}

fn part_two(input: &str) -> usize {
    let mut inclusive_count = 0;
    for line in input.lines() {
        let range: Range = line.parse().unwrap();
        if (range.first_start <= range.second_start && range.first_end >= range.second_start)
            || (range.first_start >= range.second_start && range.first_start <= range.second_end)
        {
            inclusive_count += 1;
        }
    }
    inclusive_count
}

fn main() {
    let input = include_str!("../../input_day4.txt");
    let part_one = part_one(input);
    println!("{part_one}");
    let part_two = part_two(input);
    println!("{part_two}");
}
