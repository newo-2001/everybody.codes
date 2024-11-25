#![feature(
    iter_array_chunks,
    unsigned_is_multiple_of,
    iter_collect_into
)]

use std::{error::Error, fs::File};

use jikan::{Day, DayManifest, ManifestProvider, Puzzle};

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

struct Manifests;
impl ManifestProvider for Manifests {
    fn get_manifest(day: Day) -> Result<DayManifest, Box<dyn Error>> {
        let path = format!("data/{}/day_{:02}.yaml", day.year, day.day);
        let file = File::open(&path)?;
        let manifest = serde_yml::from_reader(file)?;
        Ok(manifest)
    }
}

fn main() {
    let options = jikan::ExecutionOptions::from_args();
    let solvers: ahash::HashMap<Puzzle, Solver> = [
        solver!(2024, "01"),
        solver!(2024, "02"),
        solver!(2024, "03"),
        solver!(2024, "04"),
        solver!(2024, "05"),
        solver!(2024, "06", 1)
    ].into_iter()
        .flatten()
        .collect();

    jikan::execute::<Manifests, _, ahash::RandomState>(options, &solvers);
}