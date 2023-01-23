#![allow(clippy::derive_ord_xor_partial_ord)]
use std::cmp::{Ord, Ordering};
use std::convert::From;
use std::str::FromStr;

#[derive(Eq, PartialEq, PartialOrd, Clone)]
enum Game {
    Rock,
    Paper,
    Scissors,
}
fn parse_game(c: &str) -> Result<Game, &'static str> {
    match c {
        "A" | "X" => Ok(Game::Rock),
        "B" | "Y" => Ok(Game::Paper),
        "C" | "Z" => Ok(Game::Scissors),
        _ => Err("Unexpected character encountered"),
    }
}

impl From<Game> for usize {
    fn from(value: Game) -> Self {
        match value {
            Game::Rock => 1,
            Game::Paper => 2,
            Game::Scissors => 3,
        }
    }
}

impl Ord for Game {
    fn cmp(&self, other: &Self) -> Ordering {
        if self == other {
            return Ordering::Equal;
        }

        match self {
            Game::Rock => match other {
                Game::Paper => Ordering::Less,
                Game::Scissors => Ordering::Greater,
                _ => Ordering::Equal,
            },
            Game::Paper => match other {
                Game::Rock => Ordering::Greater,
                Game::Scissors => Ordering::Less,
                _ => Ordering::Equal,
            },
            Game::Scissors => match other {
                Game::Rock => Ordering::Less,
                Game::Paper => Ordering::Greater,
                _ => Ordering::Equal,
            },
        }
    }
}
struct Match(Game, Game);

impl FromStr for Match {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let f: Vec<&str> = s.split(' ').collect();
        return Ok(Match(parse_game(f[0]).unwrap(), parse_game(f[1]).unwrap()));
    }
}
fn part_one(input: &str) -> usize {
    let mut totals: Vec<usize> = vec![];
    for line in input.lines() {
        let game: Match = line.parse().unwrap();
        let your_score: usize = game.1.clone().into();
        let game_result = match game.1.cmp(&game.0) {
            Ordering::Less => 0,
            Ordering::Greater => 6,
            Ordering::Equal => 3,
        };
        totals.push(game_result + your_score);
    }
    return totals.iter().sum::<usize>();
}

enum Outcome {
    Win,
    Draw,
    Lose,
}
impl FromStr for Outcome {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Lose),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => Err("Unexepected outcome"),
        }
    }
}

impl From<&Outcome> for usize {
    fn from(value: &Outcome) -> Self {
        match value {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Lose => 0,
        }
    }
}

fn workout_move(opponent_move: &Game, outcome: &Outcome) -> Game {
    match outcome {
        Outcome::Win => match opponent_move {
            Game::Rock => Game::Paper,
            Game::Paper => Game::Scissors,
            Game::Scissors => Game::Rock,
        },
        Outcome::Lose => match opponent_move {
            Game::Rock => Game::Scissors,
            Game::Paper => Game::Rock,
            Game::Scissors => Game::Paper,
        },
        Outcome::Draw => opponent_move.clone(),
    }
}

fn part_two(input: &str) -> usize {
    let mut total_score = 0;
    for line in input.lines() {
        let codes = line.split(' ').collect::<Vec<&str>>();
        let opponent_move = parse_game(codes[0]).unwrap();
        let outcome: Outcome = codes[1].parse().unwrap();
        let my_move = workout_move(&opponent_move, &outcome);
        let my_score: usize = my_move.into();
        let game_score: usize = (&outcome).into();
        total_score += my_score + game_score;
    }
    total_score
}
fn main() {
    let input = include_str!("../../input_day2.txt");
    let total_score = part_one(input);
    println!("{total_score}");
    let total_score = part_two(input);
    println!("{total_score}");
}
