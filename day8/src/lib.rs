use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, multispace1},
    combinator::map,
    error::Error,
    multi::fold_many1,
    sequence::{delimited, separated_pair, terminated},
    IResult,
};
use num::integer::lcm;

#[derive(Debug)]
struct Documents {
    directions: Vec<char>,
    paths: HashMap<String, (String, String)>,
}

fn paths(text: &str) -> IResult<&str, HashMap<String, (String, String)>> {
    fold_many1(
        terminated(
            separated_pair(
                alphanumeric1::<&str, Error<&str>>,
                tag(" = "),
                delimited(
                    tag("("),
                    separated_pair(alphanumeric1, tag(", "), alphanumeric1),
                    tag(")"),
                ),
            ),
            multispace1,
        ),
        HashMap::new,
        |mut acc, (loc, (left, right))| {
            acc.insert(loc.to_string(), (left.to_string(), right.to_string()));
            acc
        },
    )(text)
}

fn documents(text: &str) -> IResult<&str, Documents> {
    map(
        separated_pair(alpha1, multispace1, paths),
        |(directions, paths)| Documents {
            directions: directions.chars().collect(),
            paths,
        },
    )(text)
}

pub fn part1(text: String) -> usize {
    let (_, documents) = documents(text.as_str()).unwrap();
    let mut cur = "AAA";
    let mut steps = 0;
    while cur != "ZZZ" {
        let dir = documents
            .directions
            .get(steps % documents.directions.len())
            .unwrap();
        match *dir {
            'L' => cur = documents.paths.get(cur).unwrap().0.as_str(),
            'R' => cur = documents.paths.get(cur).unwrap().1.as_str(),
            _ => panic!("Non-L/R char in directions"),
        }
        steps += 1;
    }
    steps
}

pub fn part2(text: String) -> usize {
    let (_, documents) = documents(text.as_str()).unwrap();
    let mut curs = documents
        .paths
        .keys()
        .filter_map(|k| {
            if k.ends_with('A') {
                Some(k.clone())
            } else {
                None
            }
        })
        .collect::<Vec<String>>();

    // Calculate cycle length (happens to num steps to node ending in 'Z')
    // TODO: slowly loosen assumptions:
    // - Actually find cycles in the cartesian product of directions and nodes
    // - Implement a chinese remainder algorithm in case of offset before the cycle
    // - If there was more than 1 goal node in the cycle, run the remainder algorithm on the
    //   cartesian product of all goal node step lengths and take the min
    let cycle_lengths = curs
        .iter_mut()
        .map(|cur| {
            let mut steps = 0;
            while !cur.ends_with('Z') {
                let dir = documents
                    .directions
                    .get(steps % documents.directions.len())
                    .unwrap();
                match *dir {
                    'L' => *cur = documents.paths.get(&cur.clone()).unwrap().0.clone(),
                    'R' => *cur = documents.paths.get(&cur.clone()).unwrap().1.clone(),
                    _ => panic!("Non-L/R char in directions"),
                }
                steps += 1;
            }
            steps
        })
        .collect::<Vec<usize>>();

    cycle_lengths
        .into_iter()
        .fold(1, |acc, length| lcm(acc, length))
}
