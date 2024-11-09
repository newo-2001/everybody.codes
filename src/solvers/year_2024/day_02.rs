use crate::SolverResult;
use everybody_helps::parsing::{parse, Parsable, ParsingResult};
use nom::{bytes::complete::tag, character::complete::{alpha1, char, line_ending}, combinator::rest, multi::{count, separated_list0}, sequence::{preceded, separated_pair}, Parser};

#[derive(Debug)]
struct RunicText<'a> {
    runic_words: Vec<&'a str>,
    inscription: &'a str
}

impl<'a> Parsable<'a> for RunicText<'a> {
    fn parse(input: &'a str) -> ParsingResult<'a, Self> {
        separated_pair(
            preceded(
                tag("WORDS:"),
                separated_list0(
                    char(','),
                    alpha1
                )
            ),
            count(line_ending, 2),
            rest
        ).map(|(runic_words, inscription)| Self {
            runic_words, inscription
        }).parse(input)
    }
}

impl RunicText<'_> {
    fn runic_words(&self) -> impl Iterator<Item=&str> {
        self.runic_words
            .iter()
            .flat_map(|word| self.inscription.matches(word))
    }
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let runic_text = parse::<RunicText>(input)?;
    Ok(Box::new(runic_text.runic_words().count()))
}