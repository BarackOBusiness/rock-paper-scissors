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
    // Error handling should be done by the user of this method.
    // For that reason a loop is not going to be implemented in the definition.
    pub fn resolve_from(process: &str) -> Self {
        match process {
            r if r == "rock" => HandShape::Rock,
            p if p == "paper" => HandShape::Paper,
            p if p == "scissor" || p == "scissors" => HandShape::Scissors,
            _ => panic!("function 'resolve_from' got an input that it should not have"),
        }
    }

    pub fn random() -> Self {
        let mut rng = rand::thread_rng();

        match rng.gen_range(0.0..3.0) {
            r if r < 1.0 => HandShape::Rock,
            p if p < 2.0 => HandShape::Paper,
            s if s < 3.0 => HandShape::Scissors,
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

// TODO: This trait
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
