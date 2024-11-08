#![feature(iter_array_chunks)]

use std::{collections::HashMap, fs::File};

use jikan::{Puzzle, Solver};

mod solvers_2024;

type SolverResult = jikan::SolverResult<anyhow::Error>;

macro_rules! solver {
    ($year: literal, $day: literal, $part: literal) => {
        (
            Puzzle { year: $year, day: $day.parse().unwrap(), part: $part },
            paste::expr! { [<solvers_ $year>]::[<day_ $day>]::[<solve_part_ $part>] } as Solver<anyhow::Error>
        )
    };
}

fn main() -> anyhow::Result<()> {
    let options = jikan::ExecutionOptions::from_args();

    let file = File::open("data.yaml")?;
    let data = serde_yml::from_reader(file)?;

    let solvers: HashMap<Puzzle, Solver<anyhow::Error>> = [
        solver!(2024, "01", 1),
        solver!(2024, "01", 2),
        solver!(2024, "01", 3)
    ].into_iter().collect();

    let manifest = jikan::Manifest { solvers, data };
    jikan::execute(options, &manifest);

    Ok(())
}