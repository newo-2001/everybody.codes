use nom::combinator::complete;
use anyhow::{anyhow, Result};

use crate::tuples::snd;

pub type ParsingError<'a> = nom::error::Error<&'a str>;
pub type ParsingResult<'a, T> = Result<(&'a str, T), nom::Err<ParsingError<'a>>>;
pub trait Parser<'a, T> = nom::Parser<&'a str, T, ParsingError<'a>>;

pub trait Parsable<'a>: Sized {
    fn parse(input: &'a str) -> ParsingResult<'a, Self>;
}

pub fn parse<'a, T: Parsable<'a>>(input: &'a str) -> Result<T> {
    run_parser(T::parse, input)
}

pub fn run_parser<'a, T, P>(parser: P, input: &'a str) -> Result<T> where
    P: Parser<'a, T>
{
    complete(parser)
        .parse(input)
        .map(snd)
        .map_err(|err| anyhow!(err.to_string()))
}