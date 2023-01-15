use std::io;

fn main() -> io::Result<()> {
    let input = include_str!("../../input_day1.txt");
    let (p1_answer, mut totals) = part_one(input);
    println!("{p1_answer}");
    let p2_answer = part_two(&mut totals);
    println!("{p2_answer}");
    Ok(())
}

fn part_one(input: &str) -> (usize, Vec<usize>) {
    let mut totals: Vec<usize> = vec![];
    let mut current: usize = 0;
    for line in input.lines() {
        if line.is_empty() {
            totals.push(current);
            current = 0;
        } else {
            let line_calorie: usize = line.parse().expect("Invalid calorie value");
            current += line_calorie;
        }
    }
    return (*totals.iter().max().unwrap(), totals);
}

fn part_two(totals: &mut [usize]) -> usize {
    totals.sort();
    totals.reverse();
    let top_three = &totals[0..=2];
    return top_three.iter().sum();
}
