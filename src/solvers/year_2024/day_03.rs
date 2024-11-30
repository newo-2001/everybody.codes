use anyhow::Result;
use yuki::{iterators::ExtraIter, parsing::run_parser, spatial::{direction::{self, Directions}, matrix::Matrix}};
use nom::{branch::alt, character::complete::{line_ending, char}, combinator::value, multi::{many1, separated_list0}};

use crate::SolverResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum Tile {
    #[default]
    Surface,
    Mine(u32)
}

impl Tile {
    const fn depth(self) -> u32 {
        match self {
            Self::Surface => 0,
            Self::Mine(depth) => depth
        }
    }
}

fn parse_grid(input: &str) -> Result<Matrix<Tile>> {
    let parser = separated_list0(
        line_ending,
        many1(alt((
            value(Tile::Surface, char('.')),
            value(Tile::Mine(0), char('#'))
        )))
    );
    
    let grid = run_parser(parser, input)?
        .into_iter()
        .try_collecting()?;

    Ok(grid)
}

fn dig<D: Directions>(grid: &Matrix<Tile>) -> Matrix<Tile> {
    grid.map(|(pos, &tile)| match tile {
        Tile::Mine(depth) if D::all()
            .map(|direction| (pos + direction)
                .and_then(|pos| grid.get(pos))
                .copied()
                .unwrap_or_default()
                .depth()
            )
            .all(|d| d == depth) => Tile::Mine(depth + 1),
        Tile::Mine(depth) => Tile::Mine(depth),
        Tile::Surface => Tile::Surface,
    })
}

fn hole_dimensions<D: Directions>(mut grid: Matrix<Tile>) -> u32 {
    loop {
        let dug = dig::<D>(&grid);
        if grid == dug { break; }
        grid = dug;
    }

    grid
        .into_iter()
        .map(Tile::depth)
        .sum()
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let grid = parse_grid(input)?;
    Ok(Box::new(hole_dimensions::<direction::Cardinal>(grid)))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let grid = parse_grid(input)?;
    Ok(Box::new(hole_dimensions::<direction::Cardinal>(grid)))
}

pub fn solve_part_3(input: &str) -> SolverResult {
    let grid = parse_grid(input)?;
    Ok(Box::new(hole_dimensions::<direction::Compass>(grid)))
}