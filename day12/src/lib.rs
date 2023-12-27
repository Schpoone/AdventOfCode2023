use core::fmt;
use indicatif::ParallelProgressIterator;
use itertools::{
    chain, repeat_n,
    FoldWhile::{Continue, Done},
    Itertools,
};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::{cmp::Ordering, collections::BinaryHeap, iter::zip};

use nom::{
    branch::alt,
    character::complete::{char, space1, u32},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult,
};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

#[derive(PartialEq, Eq)]
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

/// Test if the row filled in from the left so far is possibly valid
/// Returns the reduced row that is still unknown
/// Should not be called with fully known Rows
fn is_left_valid(row: Row) -> Option<Row> {
    // If there's not enough unknowns left to match the nums, it's invalid
    if row
        .springs
        .iter()
        .filter(|&&s| s == Spring::Damaged || s == Spring::Unknown)
        .count()
        < row.nums.iter().sum()
    {
        return None;
    }

    // If there's already too many damaged springs, it's invalid
    if row
        .springs
        .iter()
        .filter(|&&s| s == Spring::Damaged)
        .count()
        > row.nums.iter().sum()
    {
        return None;
    }

    let damaged_slices = row
        .springs
        .split(|&s| s == Spring::Operational)
        .filter(|slice| !slice.is_empty())
        .collect_vec();

    let zipped_iter = zip(row.nums.iter(), damaged_slices.clone());
    let mut unknown_idx = 0;
    for (i, (&num, damaged_slice)) in zipped_iter.enumerate() {
        unknown_idx = i;
        if damaged_slice.contains(&Spring::Unknown) {
            let known_slice = damaged_slice
                .split(|&s| s == Spring::Unknown)
                .next()
                .unwrap();
            if known_slice.len() > num {
                return None;
            } else {
                break;
            }
        }
        if num != damaged_slice.len() {
            return None;
        }
    }

    // This is the position of the beginning of the first slice that contains unknowns
    let unknown_slice_start = row
        .springs
        .split(|&s| s == Spring::Operational)
        .fold_while(0, |acc, slice| {
            if slice.contains(&Spring::Unknown) {
                Done(acc)
            } else {
                Continue(acc + slice.len() + 1)
            }
        })
        .into_inner();

    if unknown_idx == row.nums.len() - 1 {
        return Some(row);
    }

    // Figure out if there's enough unknowns left to fit the damaged springs
    let damaged_springs_needed = row.nums[unknown_idx..].iter().sum();
    if row.springs[unknown_slice_start..]
        .iter()
        .filter(|&s| *s == Spring::Unknown || *s == Spring::Damaged)
        .count()
        < damaged_springs_needed
    {
        return None;
    }

    // Figure out if there's enough space left in the row to fit the damaged springs with spaces
    let space_needed = damaged_springs_needed + row.nums[unknown_idx..].len() - 1;
    let space_left = row.springs.len() - unknown_slice_start;
    // if space_left >= space_needed {
    //     dbg!(damaged_slices);
    //     dbg!(Row {
    //         springs: row.springs.to_vec(),
    //         nums: row.nums.to_vec()
    //     });
    //     dbg!(unknown_idx);
    //     dbg!(unknown_slice_start);
    //     dbg!(space_needed);
    //     dbg!(space_left);
    // }
    if space_left >= space_needed {
        // return Some(row);
        return Some(Row {
            springs: row.springs[unknown_slice_start..].to_vec(),
            nums: row.nums[unknown_idx..].to_vec(),
        });
    } else {
        None
    }
}

/// Given a row, fill in all springs that are guaranteed
fn reason(row: Row) -> Row {
    let nums_needed_space = row.nums.iter().sum::<usize>() + row.nums.len() - 1;
    let overlapping_space = row.springs.len() - nums_needed_space;
    let mut new_springs = Vec::new();
    let mut idx = 0;
    for num in row.nums.iter() {
        if overlapping_space < *num {
            // Some springs can be filled in
            let space_to_fill = *num - overlapping_space;
            for _ in 0..(*num - space_to_fill) {
                new_springs.push(row.springs[idx]);
                idx += 1;
            }
            for _ in 0..space_to_fill {
                new_springs.push(Spring::Damaged);
                idx += 1;
            }
        } else {
            // No springs can be filled in
            for i in 0..*num {
                new_springs.push(row.springs[idx + i]);
            }
            idx += num;
        }
        if idx < row.springs.len() {
            new_springs.push(row.springs[idx]);
            idx += 1;
        }
    }
    while idx < row.springs.len() {
        new_springs.push(row.springs[idx]);
        idx += 1;
    }
    Row {
        springs: new_springs,
        nums: row.nums,
    }
}

/// Fill in one unknown from the given
fn fill_left(row: &Row, spring_type: Spring) -> Row {
    let mut springs = Vec::new();
    let mut matched = false;
    for spring in row.springs.iter() {
        if matched {
            springs.push(*spring);
        } else {
            springs.push(match spring {
                Spring::Unknown => {
                    matched = true;
                    spring_type
                }
                _ => *spring,
            })
        }
    }
    Row {
        springs,
        nums: row.nums.clone(),
    }
}

pub fn part2(text: String) -> usize {
    text.lines()
        .collect_vec()
        .par_iter()
        .progress()
        .map(|line| {
            let mut valid_line_arrangements = 0;
            // dbg!(line);
            let (_, row) = parse_line(line).unwrap();
            let row = expand(row);
            let mut possibilities = BinaryHeap::with_capacity(1024);
            // let mut printed = false;
            // let mut invalid_line_arrangements = 0;
            possibilities.push(reason(row));
            while !possibilities.is_empty() {
                let possibility = possibilities.pop().unwrap();
                let new_possibility = fill_left(&possibility, Spring::Operational);
                if new_possibility.springs.contains(&Spring::Unknown) {
                    if let Some(new_possibility) = is_left_valid(new_possibility) {
                        possibilities.push(new_possibility);
                    // } else {
                        // invalid_line_arrangements += 1;
                    }
                } else {
                    if is_valid(&new_possibility.springs, &new_possibility.nums) {
                        valid_line_arrangements += 1;
                    // } else {
                    //     invalid_line_arrangements += 1;
                    }
                }
                let new_possibility = fill_left(&possibility, Spring::Damaged);
                if new_possibility.springs.contains(&Spring::Unknown) {
                    if let Some(new_possibility) = is_left_valid(new_possibility) {
                        possibilities.push(new_possibility);
                    // } else {
                    //     invalid_line_arrangements += 1;
                    }
                } else {
                    if is_valid(&new_possibility.springs, &new_possibility.nums) {
                        valid_line_arrangements += 1;
                    // } else {
                    //     invalid_line_arrangements += 1;
                    }
                }
                // if valid_line_arrangements + invalid_line_arrangements % 100000 == 0 {
                //     dbg!(valid_line_arrangements + invalid_line_arrangements);
                //     dbg!(possibilities.peek().unwrap());
                // }
            }
            // dbg!(valid_line_arrangements);
            valid_line_arrangements
        })
        .sum()
}
