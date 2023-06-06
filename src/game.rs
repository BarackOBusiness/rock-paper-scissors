use rand::prelude::*;
use std::cmp::Ordering;

pub struct Player {
    name: String,
    score: u32,
    pub selection: HandShape,
}

#[derive(PartialEq)]
pub enum HandShape {
    Rock,
    Paper,
    Scissors,
}

impl Player {
    pub fn new(nm: &String) -> Self {
        Player {
            name: nm.to_string(),
            score: 0,
            selection: HandShape::Rock,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn score(&self) -> u32 {
        self.score
    }

    pub fn select(&mut self, sign: HandShape) {
        self.selection = sign;
    }

    pub fn incr_score(&mut self) {
        self.score += 1
    }
}

impl HandShape {
    pub fn resolve_from(input: &str) -> Result<Self, String> {
        match input {
            a if a == "rock" => Ok(HandShape::Rock),
            a if a == "paper" => Ok(HandShape::Paper),
            a if a == "scissor" || a == "scissors" => Ok(HandShape::Scissors),
            e => Err(format!(
                "You typed {e}, which is an invalid choice, valid choices are one of the following:
                rock, paper, or scissors"
            )),
        }
    }

    pub fn random() -> Self {
        let rn: u8 = rand::thread_rng().gen_range(0..3);

        match rn {
            0 => HandShape::Rock,
            1 => HandShape::Paper,
            2 => HandShape::Scissors,
            _ => panic!("Random number generator produced a value outside of possible range"),
        }
    }

    fn compare(&self, other: &HandShape) -> u8 {
        if self.eq(other) {
            return 1;
        }

        match self {
            HandShape::Rock => {
                if other.eq(&HandShape::Paper) {
                    return 0;
                } else {
                    return 2;
                }
            }
            HandShape::Paper => {
                if other.eq(&HandShape::Scissors) {
                    return 0;
                } else {
                    return 2;
                }
            }
            HandShape::Scissors => {
                if other.eq(&HandShape::Rock) {
                    return 0;
                } else {
                    return 2;
                }
            }
        }
    }
}

impl PartialOrd for HandShape {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.compare(other).partial_cmp(&(other.compare(self)))
    }
}

impl core::fmt::Display for HandShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                HandShape::Rock => "rock",
                HandShape::Paper => "paper",
                HandShape::Scissors => "scissors",
            }
        )
    }
}
