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

fn main() {
    let input = include_str!("../../input_day3.txt");
    let part_one = part_one(input);
    println!("{part_one}");
}
