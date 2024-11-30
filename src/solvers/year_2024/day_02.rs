use crate::SolverResult;
use yuki::{iterators::ExtraIter, parsing::{run_parser, ParsingResult}, spatial::{matrix::Matrix, Point}};
use itertools::Itertools;
use nom::{bytes::complete::tag, character::complete::{alpha1, char, line_ending}, combinator::rest, multi::{count, separated_list0}, sequence::{preceded, separated_pair}, Parser};

fn parse_runic_query(input: &str) -> ParsingResult<(Vec<&str>, &str)> {
    separated_pair(
        preceded(
            tag("WORDS:"),
            separated_list0(char(','), alpha1)
        ),
        count(line_ending, 2),
        rest
    ).parse(input)
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let (runic_words, runic_text) = run_parser(parse_runic_query, input)?;
    let runic_word_count = runic_words
        .into_iter()
        .flat_map(|word| runic_text.matches(word))
        .count();

    Ok(Box::new(runic_word_count))
}

fn extract_runes<'a>(
    text: &'a str,
    runic_words: &'a [&'a str]
) -> impl Iterator<Item=(usize, usize)> + 'a where {
    runic_words
        .iter()
        .flat_map(|word| text.match_indices(word))
        .map(|(start, r#match)| (start, r#match.len()))
}

fn extract_bidirectional_runes<'a>(
    text: &'a str,
    runic_words: &'a [&'a str]
) -> impl Iterator<Item=usize> + 'a {
    let reversed: String = text.chars().rev().collect();

    extract_runes(reversed.as_str(), runic_words)
        .map(|(start, length)| (text.len() - start - length, length))
        .chain(extract_runes(text, runic_words))
        .flat_map(|(start, length)| start..start + length)
        .collect_vec()
        .into_iter()
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let (runic_words, runic_text) = run_parser(parse_runic_query, input)?;

    let runes = extract_bidirectional_runes(runic_text, &runic_words)
        .unique()
        .count();

    Ok(Box::new(runes))
}

pub fn solve_part_3(input: &str) -> SolverResult {
    let (runic_words, runic_text) = run_parser(parse_runic_query, input)?;

    let scales: Matrix<char> = runic_text
        .lines()
        .map(|line| line.bytes().map(|byte| byte as char))
        .try_collecting()?;

    let vertical_runes = scales
        .iter_cols()
        .enumerate()
        .flat_map(|(x, col)| {
            let str = col.collect::<String>();
            extract_bidirectional_runes(&str, &runic_words)
                .map(|y| Point { x, y })
                .collect_vec()
        });

    let horizontal_runes = scales
        .iter_rows()
        .enumerate()
        .flat_map(|(y, row)| {
            let str = String::from_iter(row).repeat(2);
            extract_bidirectional_runes(&str, &runic_words)
                .map(|x| Point { x: x % scales.cols(), y })
                .collect_vec()
        });

    let runes = horizontal_runes
        .chain(vertical_runes)
        .unique()
        .count();

    Ok(Box::new(runes))
}