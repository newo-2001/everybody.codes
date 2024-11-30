use std::{collections::VecDeque, fmt::Display, iter::once};

use ahash::HashMap;
use anyhow::{anyhow, Context, Result};
use itertools::Itertools;
use nom::{character::complete::{alpha1, char, line_ending}, combinator::{map, value}, multi::separated_list1, sequence::separated_pair, Parser};
use yuki::parsing::{parse, Parsable, ParsingResult};

use crate::SolverResult;

#[derive(Debug, Clone, Copy)]
enum Child<'a> {
    Fruit,
    Node(&'a str)
}

impl<'a> Parsable<'a> for Child<'a> {
    fn parse(input: &'a str) -> ParsingResult<'a, Self> {
        Parser::or(
            value(Self::Fruit, char('@')),
            map(alpha1, Self::Node)
        ).parse(input)
    }
}

struct Node<'a> {
    name: &'a str,
    children: Vec<Child<'a>>
}

impl<'a> Parsable<'a> for Node<'a> {
    fn parse(input: &'a str) -> ParsingResult<'a, Self> {
        separated_pair(
            alpha1,
            char(':'),
            separated_list1(
                char(','),
                Child::parse
            )
        )
        .map(|(name, children)| Self { name, children })
        .parse(input)
    }
}

const ROOT: &str = "RR";
struct Tree<'a>(HashMap<&'a str, Node<'a>>);

impl<'a> Parsable<'a> for Tree<'a> {
    fn parse(input: &'a str) -> ParsingResult<'a, Self> {
        separated_list1(
            line_ending,
            Node::parse
        )
        .map(|nodes| nodes
            .into_iter()
            .map(|node| (node.name, node))
            .collect()
        )
        .map(Tree)
        .parse(input)
    }
}

#[derive(Debug, Default, Clone)]
struct Path<'a>(Vec<&'a str>);

impl Display for Path<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for node in &self.0 {
            write!(f, "{node}")?;
        }

        Ok(())
    }
}

struct FruitIterator<'a, 'b> {
    queue: VecDeque<(Child<'a>, Path<'a>)>,
    tree: &'b Tree<'a>
}

impl<'a> Iterator for FruitIterator<'a, '_> {
    type Item = Path<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (next, mut path) = self.queue.pop_front()?;
            match next {
                Child::Fruit => {
                    path.0.push("@");
                    return Some(path);
                },
                Child::Node(name) => match self.tree.0.get(name) {
                    None => continue,
                    Some(node) => {
                        let mut path = path.clone();
                        path.0.push(name);

                        node.children
                            .iter()
                            .map(|&child| (child, path.clone()))
                            .collect_into(&mut self.queue);
                    }
                }
            }
        }
    }
}

impl<'a> Tree<'a> {
    fn fruits<'b>(&'b self) -> FruitIterator<'a, 'b> {
        let queue = once((Child::Node(ROOT), Path::default())).collect();
        FruitIterator { queue, tree: self }
    }

    fn most_powerful_fruit(&self) -> Result<Path<'a>> {
        self
            .fruits()
            .map(|path| (path.0.len(), path))
            .into_group_map()
            .into_values()
            .min_by_key(Vec::len)
            .context("No fruits in input")?
            .into_iter()
            .exactly_one()
            .map_err(|err| anyhow!(err.to_string()))
    }
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let tree: Tree = parse(input)?;
    let path = tree.most_powerful_fruit()?;
    Ok(Box::new(path.to_string()))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let tree: Tree = parse(input)?;
    let path = tree.most_powerful_fruit()?.0
        .into_iter()
        .filter_map(|str| str.chars().next())
        .join("");

    Ok(Box::new(path))
}

pub fn solve_part_3(input: &str) -> SolverResult {
    solve_part_2(input)
}