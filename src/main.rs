use std::collections::HashMap;

use jikan::{ExecutionOptions, Puzzle, Solver};

mod solvers_2024;

macro_rules! solver {
    ($year: literal, $day: literal, $part: literal) => {
        (
            Puzzle { year: $year, day: $day.parse().unwrap(), part: $part },
            paste::expr! { [<solvers_ $year>]::[<day_ $day>]::[<solve_part_ $part>] } as Solver<anyhow::Error>
        )
    };
}

fn main() -> Result<(), jikan::Error> {
    let options = ExecutionOptions::from_args()?;
    let solvers: HashMap<Puzzle, Solver<anyhow::Error>> = [
        solver!(2024, "01", 1)
    ].into_iter().collect();

    jikan::execute(options, &solvers);

    Ok(())
}