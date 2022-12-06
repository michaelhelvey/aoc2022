#![allow(dead_code)]

use color_eyre::eyre::Result;
use std::{convert::TryInto, str::FromStr};

#[derive(PartialEq, Debug, Copy, Clone)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl TryFrom<i32> for Move {
    type Error = color_eyre::Report;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Rock),
            2 => Ok(Self::Paper),
            3 => Ok(Self::Scissors),
            _ => Err(color_eyre::eyre::eyre!("invalid enum value {}", value)),
        }
    }
}

impl TryFrom<char> for Move {
    type Error = color_eyre::Report;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'A' => Ok(Self::Rock),
            'B' => Ok(Self::Paper),
            'C' => Ok(Self::Scissors),
            _ => Err(color_eyre::eyre::eyre!("not a valid move: {}", c)),
        }
    }
}

impl Move {
    fn score(self: &Self) -> i32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn beats(self: &Self, other: Move) -> bool {
        matches!(
            (self, other),
            (Move::Rock, Move::Scissors)
                | (Move::Scissors, Move::Paper)
                | (Move::Paper, Move::Rock)
        )
    }

    fn move_beating(other: Move) -> Move {
        match other {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock,
        }
    }

    fn move_losing_to(other: Move) -> Move {
        match other {
            Self::Paper => Self::Rock,
            Self::Scissors => Self::Paper,
            Self::Rock => Self::Scissors,
        }
    }
}

struct Round {
    expected_result: RoundResult,
    theirs: Move,
}

#[derive(Debug, PartialEq)]
enum RoundResult {
    Victory,
    Loss,
    Draw,
}

impl RoundResult {
    fn score(self: &Self) -> i32 {
        match self {
            Self::Loss => 0,
            Self::Draw => 3,
            Self::Victory => 6,
        }
    }
}

impl Round {
    fn new(theirs: Move, expected_result: RoundResult) -> Self {
        Self {
            expected_result,
            theirs,
        }
    }

    fn score(self: &Self) -> i32 {
        self.our_move().score() + self.expected_result.score()
    }

    fn our_move(self: &Self) -> Move {
        match self.expected_result {
            RoundResult::Victory => Move::move_beating(self.theirs),
            RoundResult::Loss => Move::move_losing_to(self.theirs),
            RoundResult::Draw => self.theirs,
        }
    }
}

impl TryFrom<char> for RoundResult {
    type Error = color_eyre::Report;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'X' => Ok(Self::Loss),
            'Y' => Ok(Self::Draw),
            'Z' => Ok(Self::Victory),
            _ => Err(color_eyre::eyre::eyre!(
                "cannot parse RoundResult from {}",
                c
            )),
        }
    }
}

impl FromStr for Round {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let (Some(theirs), Some(' '), Some(expected_result)) = (chars.next(), chars.next(), chars.next()) else {
            return Err(color_eyre::eyre::eyre!("Expected <theirs>space<expected_result>; received {}", s));
        };

        Ok(Self {
            expected_result: expected_result.try_into()?,
            theirs: theirs.try_into()?,
        })
    }
}

pub fn run() -> Result<()> {
    let input = include_str!("./inputs/day_2.txt");

    let total_score: i32 = input
        .lines()
        .map(|line| line.parse::<Round>())
        .map(|round| round.unwrap().score())
        .sum();

    println!("total score: {}", total_score);

    Ok(())
}
