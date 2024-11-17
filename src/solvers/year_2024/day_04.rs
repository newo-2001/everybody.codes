use crate::SolverResult;
use anyhow::{Context, Result};

fn parse_nails(input: &str) -> Result<Vec<u32>> {
    let nails = input
        .lines()
        .map(str::parse)
        .collect::<Result<_, _>>()?;

    Ok(nails)
}

fn min_strikes(nails: &[u32]) -> Result<u32> {
    let min = nails
        .iter()
        .copied()
        .min()
        .context("No nails in input")?;

    let strikes = nails
        .iter()
        .copied()
        .map(|nail| nail - min)
        .sum();

    Ok(strikes)
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let nails = parse_nails(input)?;
    Ok(Box::new(min_strikes(&nails)?))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let nails = parse_nails(input)?;
    Ok(Box::new(min_strikes(&nails)?))
}

fn min_actions(nails: &[u32]) -> Result<u32> {
    let actions = nails
        .iter()
        .map(|&pivot| nails
            .iter()
            .copied()
            .map(|nail| nail.abs_diff(pivot))
            .sum()
        )
        .min()
        .context("No nails in input")?;

    Ok(actions)
}

pub fn solve_part_3(input: &str) -> SolverResult {
    let nails = parse_nails(input)?;
    Ok(Box::new(min_actions(&nails)?))
}