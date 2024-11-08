use anyhow::{bail, Result};

use crate::SolverResult;

#[derive(Debug, Clone, Copy)]
enum Creature {
    AncientAnt,
    BadassBeetle,
    CreepyCockroach,
    DiabolicalDragonfly
}

impl Creature {
    fn parse(value: char) -> Result<Option<Self>> {
        Ok(Some(match value {
            'A' => Self::AncientAnt,
            'B' => Self::BadassBeetle,
            'C' => Self::CreepyCockroach,
            'D' => Self::DiabolicalDragonfly,
            'x' => return Ok(None),
            _ => bail!("Invalid creature: {value}")
        }))
    }

    fn cost(self) -> u32 {
        match self {
            Self::AncientAnt => 0,
            Self::BadassBeetle => 1,
            Self::CreepyCockroach => 3,
            Self::DiabolicalDragonfly => 5
        }
    }
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let potions: u32 = input.chars()
        .map(Creature::parse)
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .flatten()
        .map(Creature::cost)
        .sum();

    Ok(Box::new(potions))
}

#[derive(Debug, Clone, Copy)]
struct Group<'a>(&'a [Option<Creature>]);

impl Group<'_> {
    fn cost(&self) -> u32 {
        let creatures = self.0.iter().flatten();
        let base_cost: u32 = creatures
            .clone()
            .copied()
            .map(Creature::cost)
            .sum();

        #[allow(clippy::cast_possible_truncation)]
        let extra_cost = match creatures.count() as u32 {
            count if count > 2 => count * 2,
            count if count > 1 => count,
            _ => 0
        };

        base_cost + extra_cost
    }
}

fn group_fight_cost<const N: usize>(input: &str) -> Result<u32> {
    let cost = input.chars()
        .map(Creature::parse)
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .array_chunks::<N>()
        .map(|chunk| Group(&chunk).cost())
        .sum();

    Ok(cost)
}

pub fn solve_part_2(input: &str) -> SolverResult {
    Ok(Box::new(group_fight_cost::<2>(input)?))
}

pub fn solve_part_3(input: &str) -> SolverResult {
    Ok(Box::new(group_fight_cost::<3>(input)?))
}