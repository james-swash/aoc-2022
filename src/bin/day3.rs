fn part_one(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            line[0..line.len() / 2]
                .chars()
                .find_map(|x| line[line.len() / 2..].chars().find(|z| *z == x))
                .unwrap()
        })
        .map(|chr| {
            chr.is_uppercase()
                .then_some(chr)
                .map_or_else(|| chr as usize - 96, |c| c as usize - 38)
        })
        .sum()
}

fn part_two(input: &str) -> usize {
    let lines: Vec<_> = input.lines().collect();
    let chunks = lines.chunks(3);
    let mut tmp: Vec<usize> = Vec::with_capacity(chunks.len());
    for chunk in chunks {
        for a in chunk[0].chars() {
            if chunk[1].chars().any(|b| b == a) {
                if let Some(c) = chunk[2].chars().find(|c| *c == a) {
                    if c.is_uppercase() {
                        tmp.push(c as usize - 38);
                    } else {
                        tmp.push(c as usize - 96);
                    }
                    break;
                }
            }
        }
    }
    tmp.iter().sum()
}

fn main() {
    let input = include_str!("../../input_day3.txt");
    let start_1 = Instant::now();
    let part_one = part_one(input);
    let dur_1 = start_1.elapsed();
    println!("{part_one}, {dur_1:?}");
    use std::time::Instant;
    let start = Instant::now();
    let part_two = part_two(input);
    let duration = start.elapsed();
    println!("{part_two}, {duration:?}");
}
