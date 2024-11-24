use std::{cmp::max, collections::VecDeque};

use ahash::{HashMap, HashMapExt, HashSet, HashSetExt};
use anyhow::{anyhow, Result};
use everybody_helps::{iterators::ExtraIter, parsing::{run_parser, ParsingResult}, spatial::matrix::Matrix};
use itertools::Itertools;
use nom::{character::complete::{line_ending, space1, u32}, combinator::map_res, multi::separated_list1, Parser};

use crate::SolverResult;

fn parse_dance(input: &str) -> ParsingResult<Vec<VecDeque<u32>>> {
    map_res(
        separated_list1(
            line_ending,
            separated_list1(space1, u32)
        ),
        |cols| {
            let transposed = cols
                .into_iter()
                .try_collecting::<Matrix<u32>>()?
                .into_cols()
                .map(|col| col.into_iter().collect())
                .collect();

            Ok::<_, anyhow::Error>(transposed)
        }
    ).parse(input)
}

fn perform_round(configuration: &mut [VecDeque<u32>], round: usize) -> Result<()> {
    let clapper = configuration
        .get_mut(round % configuration.len())
        .unwrap()
        .pop_front()
        .ok_or_else(|| anyhow!("Column is empty"))?;

    let column_num: usize = (round + 1) % configuration.len();
    let column = &mut configuration[column_num];
    let mut row_num: usize = 0;
    let mut claps: u32 = 0;

    loop {
        let moving_down = claps as usize % (column.len() * 2) < column.len();

        claps += 1;
        if claps == clapper {
            let column = &mut configuration[column_num];
            column.insert(if moving_down { row_num } else { row_num + 1 }, clapper);
            break;
        }

        if moving_down && row_num != column.len() - 1 { row_num += 1; }
        else if !moving_down && row_num != 0 { row_num -= 1; }
    }

    Ok(())
}

fn shout_number(configuration: &[VecDeque<u32>]) -> u64 {
    configuration
        .iter()
        .map(|col| col.front().unwrap())
        .join("")
        .parse()
        .unwrap()
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let mut configuration = run_parser(parse_dance, input)?;

    for i in 0..10 {
        perform_round(&mut configuration, i)?;
    }

    Ok(Box::new(shout_number(&configuration)))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let mut configuration = run_parser(parse_dance, input)?;
    let mut seen = HashMap::<u64, u32>::new();
    let mut round: usize = 0;

    let number = loop {
        perform_round(&mut configuration, round)?;

        let number = shout_number(&configuration);
        match seen.get(&number) {
            Some(&count) if count == 2024 - 1 => break number,
            Some(&count) => seen.insert(number, count + 1),
            None => seen.insert(number, 1)
        };

        round += 1;
    };

    Ok(Box::new(number * (round as u64 + 1)))
}

pub fn solve_part_3(input: &str) -> SolverResult {
    let mut states = HashSet::<(usize, Vec<VecDeque<u32>>)>::new();
    let mut configuration = run_parser(parse_dance, input)?;
    let mut largest: Option<u64> = None;

    for round in 0.. {
        perform_round(&mut configuration, round)?;

        let number = shout_number(&configuration);
        largest = Some(largest.map_or(number, |largest| max(largest, number)));

        let col = round % configuration.len();
        let state = (col, configuration.clone());
        if !states.insert(state) {
            return Ok(Box::new(largest.unwrap()))
        };
    };

    unreachable!();
}