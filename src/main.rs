#![feature(
    iter_array_chunks
)]

use std::{collections::HashMap, fs::File};

use jikan::Puzzle;

mod solvers;

type SolverResult = jikan::SolverResult<anyhow::Error>;
type Solver = jikan::Solver<anyhow::Error>;

macro_rules! solver {
    ($year: literal, $day: literal, $part: literal) => {
        vec![(
            Puzzle { year: $year, day: $day.parse().unwrap(), part: $part },
            paste::expr! { solvers::[<year_ $year>]::[<day_ $day>]::[<solve_part_ $part>] } as Solver
        )]
    };

    ($year: literal, $day: literal) => {
        [
            solver!($year, $day, 1),
            solver!($year, $day, 2),
            solver!($year, $day, 3)
        ].into_iter()
            .flatten()
            .collect::<Vec<_>>()
    }
}

fn main() -> anyhow::Result<()> {
    let options = jikan::ExecutionOptions::from_args();

    let file = File::open("data.yaml")?;
    let data = serde_yml::from_reader(file)?;

    let solvers: HashMap<Puzzle, Solver> = [
        solver!(2024, "01"),
        solver!(2024, "02"),
        solver!(2024, "03"),
        solver!(2024, "04")
    ].into_iter()
        .flatten()
        .collect();

    let manifest = jikan::Manifest { solvers, data };
    jikan::execute(options, &manifest);

    Ok(())
}