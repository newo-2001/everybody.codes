use std::collections::VecDeque;

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
            let transposed = cols.into_iter()
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
        #[allow(clippy::cast_possible_truncation)]
        let moving_down = claps < column.len() as u32;

        row_num = row_num.checked_add_signed(if moving_down { 1 } else { -1 }).unwrap();

        claps += 1;
        if claps == clapper {
            let column = &mut configuration[column_num];
            column.insert(if moving_down { row_num - 1} else { row_num }, clapper);
            break;
        }
    }

    Ok(())
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let mut configuration = run_parser(parse_dance, input)?;

    for i in 0..10 {
        perform_round(&mut configuration, i)?;
    }

    let number = configuration
        .iter()
        .map(|col| col.front().unwrap())
        .join("");

    Ok(Box::new(number))
}