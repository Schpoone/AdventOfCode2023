use core::fmt;
use itertools::{chain, repeat_n, Itertools};
use memoize::memoize;
use std::{cmp::Ordering, iter::zip};

use nom::{
    branch::alt,
    character::complete::{char, space1, u32},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult,
};

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

#[derive(PartialEq, Eq, Clone, Hash)]
struct Row {
    springs: Vec<Spring>,
    nums: Vec<usize>,
}

impl PartialOrd for Row {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Row {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_known_depth = self
            .springs
            .iter()
            .position(|&s| s == Spring::Unknown)
            .unwrap_or(self.springs.len());
        let other_known_depth = other
            .springs
            .iter()
            .position(|&s| s == Spring::Unknown)
            .unwrap_or(other.springs.len());
        self_known_depth.cmp(&other_known_depth)
    }
}

impl fmt::Debug for Row {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {}",
            self.springs
                .iter()
                .map(|s| match s {
                    Spring::Operational => '.',
                    Spring::Damaged => '#',
                    Spring::Unknown => '?',
                })
                .collect::<String>(),
            self.nums.iter().map(|n| n.to_string()).join(",")
        )
    }
}

fn is_valid(springs: &[Spring], nums: &[usize]) -> bool {
    if springs.contains(&Spring::Unknown) {
        return false;
    }

    let damaged_slices = springs
        .split(|&s| s == Spring::Operational)
        .filter(|slice| !slice.is_empty())
        .collect_vec();
    if nums.len() != damaged_slices.len() {
        return false;
    }
    zip(nums.iter(), damaged_slices.iter()).all(|(&num, damaged_slice)| num == damaged_slice.len())
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
        separated_list1(char(','), map(u32, |n| n as usize)),
    )(text)?;
    Ok((text, Row { springs, nums }))
}

// Very much, brute forcing it
pub fn part1(text: String) -> u32 {
    let mut valid_arrangements = 0;
    for line in text.lines() {
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
            let revealed_springs = row
                .springs
                .iter()
                .map(|spring| match spring {
                    Spring::Unknown => combo.pop().unwrap(),
                    _ => *spring,
                })
                .collect_vec();
            if is_valid(&revealed_springs, &row.nums) {
                valid_arrangements += 1;
            }
        }
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
        .collect::<Vec<usize>>(),
    }
}

/// Count number of valid arrangements with a dynamic programming approach
/// Place the first block of springs on the left,
/// if the block itself is not contradicting anything,
/// call num_valid() again using a row without the block and without the first num
/// do this for every possible placement of the first block and sum them
#[memoize(SharedCache)]
fn num_valid(row: Row) -> usize {
    // If there's not enough unknowns left to match the nums, it's invalid
    if row
        .springs
        .iter()
        .filter(|&&s| s == Spring::Damaged || s == Spring::Unknown)
        .count()
        < row.nums.iter().sum()
    {
        return 0;
    }

    let num = row.nums[0];
    (0..=(row.springs.len() - num))
        .map(|start_position| {
            if row
                .springs
                .iter()
                .take(start_position + num)
                .enumerate()
                .all(|(idx, &s)| {
                    if idx < start_position {
                        s == Spring::Operational || s == Spring::Unknown
                    } else {
                        s == Spring::Damaged || s == Spring::Unknown
                    }
                })
            {
                if row.nums.len() == 1 {
                    if row.springs[start_position + num..]
                        .iter()
                        .all(|&s| s != Spring::Damaged)
                    {
                        1
                    } else {
                        0
                    }
                } else if row.springs.len() <= start_position + num + 1
                    || row.springs[start_position + num] == Spring::Damaged
                {
                    0
                } else {
                    let new_row = Row {
                        springs: row.springs[start_position + num + 1..].to_vec(),
                        nums: row.nums[1..].to_vec(),
                    };
                    num_valid(new_row)
                }
            } else {
                0
            }
        })
        .sum()
}

pub fn part2(text: String) -> usize {
    text.lines()
        .map(|line| {
            let (_, row) = parse_line(line).unwrap();
            let row = expand(row);
            num_valid(row)
        })
        .sum()
}
