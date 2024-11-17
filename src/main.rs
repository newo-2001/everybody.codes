#![feature(
    iter_array_chunks
)]

use std::{collections::HashMap, fs::File, path::Path};

use jikan::{DataManifest, Day, DayManifest, Puzzle};

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
    let puzzles: HashMap<Day, DayManifest> = jikan::locate_manifests(Path::new("data"), options.scope)?
        .into_iter()
        .map(|(day, path)| {
            let file = File::open(path)?;
            let manifest = serde_yml::from_reader(file)?;
            Ok((day, manifest))
        })
        .collect::<anyhow::Result<_>>()?;

    let solvers: HashMap<Puzzle, Solver> = [
        solver!(2024, "01"),
        solver!(2024, "02"),
        solver!(2024, "03"),
        solver!(2024, "04")
    ].into_iter()
        .flatten()
        .collect();

    let manifest = jikan::Manifest {
        data: DataManifest { puzzles },
        solvers,
    };

    jikan::execute(options, &manifest);

    Ok(())
}