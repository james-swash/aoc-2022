use std::collections::HashSet;

fn find_start(message: &str, marker_count: usize) -> usize {
    for (i, window) in message
        .chars()
        .collect::<Vec<_>>()
        .windows(marker_count)
        .enumerate()
    {
        let window_len = &window.len();
        let dedup = &window.iter().collect::<HashSet<_>>();
        if window_len == &dedup.len() {
            return i + marker_count;
        }
    }
    unreachable!("If we get here, the input is broken");
}
fn part_one(input: &str) -> usize {
    find_start(input, 4)
}

fn part_two(input: &str) -> usize {
    find_start(input, 14)
}

fn main() {
    let input = include_str!("../../input_day6.txt");
    let part_one = part_one(input);
    println!("{part_one}");
    let part_two = part_two(input);
    println!("{part_two}");
}
