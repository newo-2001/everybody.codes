use anyhow::{bail, Error, Result};
use jikan::SolverResult;

#[derive(Debug, Clone, Copy)]
enum Creature {
    AncientAnt,
    BadassBeetle,
    CreepyCockroach
}

impl Creature {
    fn cost(self) -> u32 {
        match self {
            Self::AncientAnt => 0,
            Self::BadassBeetle => 1,
            Self::CreepyCockroach => 3
        }
    }
}

impl TryFrom<char> for Creature {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            'A' => Self::AncientAnt,
            'B' => Self::BadassBeetle,
            'C' => Self::CreepyCockroach,
            _ => bail!("Invalid creature: {value}")
        })
    }
}

pub fn solve_part_1(input: &str) -> SolverResult<Error> {
    let potions: u32 = input.chars()
        .map(Creature::try_from)
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .map(Creature::cost)
        .sum();

    Ok(Box::new(potions))
}