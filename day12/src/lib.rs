use itertools::{chain, repeat_n, Itertools};
use std::iter::zip;

use nom::{
    branch::alt,
    character::complete::{char, space1, u32},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult,
};

#[derive(Debug, PartialEq, Copy, Clone)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

#[derive(Debug)]
struct Row {
    springs: Vec<Spring>,
    nums: Vec<u32>,
}

fn is_valid(springs: &Vec<Spring>, nums: &Vec<u32>) -> bool {
    if springs.contains(&Spring::Unknown) {
        return false;
    }

    let damaged_slices = springs
        .split(|&s| s == Spring::Operational)
        .filter(|slice| !slice.is_empty())
        .collect_vec();
    // dbg!(nums);
    // dbg!(&damaged_slices);
    if nums.len() != damaged_slices.len() {
        return false;
    }
    zip(
        nums.iter(),
        springs
            .split(|&s| s == Spring::Operational)
            .filter(|slice| !slice.is_empty()),
    )
    .all(|(&num, damaged_slice)| num == damaged_slice.len() as u32)
}

fn parse_line(text: &str) -> IResult<&str, Row> {
    let (text, (springs, nums)) = separated_pair(
        many1(map(alt((char('.'), char('#'), char('?'))), |c| match c {
            '.' => Spring::Operational,
            '#' => Spring::Damaged,
            '?' => Spring::Unknown,
            c => panic!("Unknown character parsed: {}", c),
        })),
        space1,
        separated_list1(char(','), u32),
    )(text)?;
    Ok((text, Row { springs, nums }))
}

// Very much, brute forcing it
pub fn part1(text: String) -> u32 {
    let mut valid_arrangements = 0;
    for line in text.lines() {
        // dbg!(line);
        let (_, row) = parse_line(line).unwrap();
        let combos = repeat_n(
            [Spring::Operational, Spring::Damaged].into_iter(),
            row.springs
                .iter()
                .filter(|&&s| s == Spring::Unknown)
                .count(),
        )
        .multi_cartesian_product();
        for mut combo in combos {
            // dbg!(&combo);
            let revealed_springs = row
                .springs
                .iter()
                .map(|spring| match spring {
                    Spring::Unknown => combo.pop().unwrap(),
                    _ => *spring,
                })
                .collect_vec();
            // dbg!(&revealed_springs);
            if is_valid(&revealed_springs, &row.nums) {
                // dbg!("Is valid");
                valid_arrangements += 1;
            }
            // println!("----------------------------")
        }
        // dbg!(valid_arrangements);
    }
    valid_arrangements
}

fn expand(row: Row) -> Row {
    let mut new_springs = row.springs.clone();
    for _ in 0..4 {
        new_springs.push(Spring::Unknown);
        new_springs.extend(row.springs.iter());
    }
    Row {
        springs: new_springs,
        nums: chain![
            row.nums.iter(),
            row.nums.iter(),
            row.nums.iter(),
            row.nums.iter(),
            row.nums.iter()
        ]
        .map(|n| *n)
        .collect::<Vec<u32>>(),
    }
}

pub fn part2(text: String) -> i32 {
    let mut valid_arrangements = 0;
    for line in text.lines() {
        // dbg!(line);
        let (_, row) = parse_line(line).unwrap();
        let row = expand(row);
        // dbg!(&row);
        let combos = repeat_n(
            [Spring::Operational, Spring::Damaged].into_iter(),
            row.springs
                .iter()
                .filter(|&&s| s == Spring::Unknown)
                .count(),
        )
        .multi_cartesian_product();
        for mut combo in combos {
            // dbg!(&combo);
            let revealed_springs = row
                .springs
                .iter()
                .map(|spring| match spring {
                    Spring::Unknown => combo.pop().unwrap(),
                    _ => *spring,
                })
                .collect_vec();
            // dbg!(&revealed_springs);
            if is_valid(&revealed_springs, &row.nums) {
                // dbg!("Is valid");
                valid_arrangements += 1;
            }
            // println!("----------------------------")
        }
        // dbg!(valid_arrangements);
    }
    valid_arrangements
}
