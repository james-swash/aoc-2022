use std::collections::HashMap;
use std::str::FromStr;

struct Crates(HashMap<usize, Vec<char>>);

impl Crates {
    fn move_onexone(&mut self, command: &Manoeuvre) {
        let inner = &mut self.0;
        for _ in 0..command.amount {
            if let Some(cargo) = inner.get_mut(&command.from).unwrap().pop() {
                inner.get_mut(&command.to).unwrap().push(cargo);
            }
        }
    }

    fn move_many(&mut self, command: &Manoeuvre) {
        let inner = &mut self.0;
        let from_stack = inner.get_mut(&command.from).unwrap();
        let split_at = if let Some(split_at) = from_stack.len().checked_sub(command.amount) {
            split_at
        } else {
            0
        };
        let removed = from_stack.split_off(split_at);
        inner
            .get_mut(&command.to)
            .unwrap()
            .extend_from_slice(&removed);
    }

    fn top_crates(&self) -> [char; 9] {
        let mut top_crates = ['A'; 9];
        for (k, v) in self.0.iter() {
            top_crates[k - 1] = v[v.len() - 1];
        }
        top_crates
    }
}

impl FromStr for Crates {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cargo_map: HashMap<usize, Vec<char>> = HashMap::new();
        for i in 1..=9 {
            cargo_map.insert(i, vec![]);
        }
        for line in s.lines().rev() {
            for (i, col) in line.chars().collect::<Vec<_>>().chunks(4).enumerate() {
                if col[1].is_alphabetic() {
                    cargo_map.get_mut(&(i + 1)).unwrap().push(col[1]);
                }
            }
        }
        Ok(Self(cargo_map))
    }
}

struct Manoeuvre {
    from: usize,
    to: usize,
    amount: usize,
}

impl FromStr for Manoeuvre {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let amount_and_move = s.split("from").collect::<Vec<_>>();
        let amount = amount_and_move[0]
            .replace("move", "")
            .trim()
            .parse::<usize>()
            .unwrap();
        let moves = amount_and_move[1].split("to").collect::<Vec<_>>();
        let from = moves[0].trim().parse::<usize>().unwrap();
        let to = moves[1].trim().parse::<usize>().unwrap();
        Ok(Self { from, to, amount })
    }
}

fn split_crates_and_moves(input: &str) -> (&str, &str) {
    let crates_and_moves = input.split("\n\n").collect::<Vec<_>>();
    (crates_and_moves[0], crates_and_moves[1])
}

fn part_one(input: &str) -> String {
    let (crates_str, moves_str) = split_crates_and_moves(input);
    let mut crates: Crates = crates_str.parse().unwrap();
    for line in moves_str.lines() {
        let command: Manoeuvre = line.parse().unwrap();
        crates.move_onexone(&command);
    }
    crates.top_crates().iter().collect::<String>()
}

fn part_two(input: &str) -> String {
    let (crates_str, moves_str) = split_crates_and_moves(input);
    let mut crates: Crates = crates_str.parse().unwrap();
    for line in moves_str.lines() {
        let command: Manoeuvre = line.parse().unwrap();
        crates.move_many(&command);
    }
    crates.top_crates().iter().collect::<String>()
}

fn main() {
    use std::time::Instant;
    let input = include_str!("../../input_day5.txt");
    let start = Instant::now();
    let part_one = part_one(input);
    let elapsed = start.elapsed();
    println!("{part_one} - {elapsed:?}");
    let start = Instant::now();
    let part_two = part_two(input);
    let elapsed = start.elapsed();
    println!("{part_two} - {elapsed:?}");
}
