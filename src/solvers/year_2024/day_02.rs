use crate::SolverResult;
use everybody_helps::parsing::{run_parser, ParsingResult};
use itertools::Itertools;
use nom::{bytes::complete::tag, character::complete::{alpha1, char, line_ending}, combinator::rest, multi::{count, separated_list0}, sequence::{preceded, separated_pair}, Parser};

struct RunicText<'a>(&'a str);

fn parse_runic_query(input: &str) -> ParsingResult<(Vec<&str>, RunicText)> {
    separated_pair(
        preceded(
            tag("WORDS:"),
            separated_list0(
                char(','),
                alpha1
            )
        ),
        count(line_ending, 2),
        rest.map(RunicText)
    ).parse(input)
}


impl<'a> RunicText<'a> {
    fn runic_words<'b, I>(&self, query: I) -> impl Iterator<Item=&'b str> where
        I: IntoIterator<Item=&'b str>,
        'a: 'b
    {
        query
            .into_iter()
            .flat_map(|word| self.0.matches(word))
    }

    fn runic_symbol_count<'b>(&self, query: impl IntoIterator<Item=&'b str>) -> usize where
    {
        let reversed: String = self.0.chars()
            .rev()
            .collect();

        query.into_iter()
            .flat_map(|word| {
                let matches = self.0
                    .match_indices(word)
                    .flat_map(|(start, word)| start..start + word.len());

                let reverse_matches = reversed
                    .match_indices(word)
                    .flat_map(|(start, word)| {
                        let start = self.0.len() - start - word.len();
                        start..start + word.len()
                    });

                matches.chain(reverse_matches)
            })
            .unique()
            .count()
    }
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let (query, runic_text) = run_parser(parse_runic_query, input)?;
    Ok(Box::new(runic_text.runic_words(query).count()))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let (query, runic_text) = run_parser(parse_runic_query, input)?;
    Ok(Box::new(runic_text.runic_symbol_count(query)))
}