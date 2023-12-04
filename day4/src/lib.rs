use nom::character::complete::*;
use nom::combinator::map;
use nom::multi::*;
use nom::sequence::*;
use nom::IResult;

use std::collections::hash_map::HashMap;

fn parse(text: &str) -> IResult<&str, (Vec<u32>, Vec<u32>)> {
    let (text, _) = many_till(anychar, pair(char(':'), space1))(text)?;
    let (text, win_nums) =
        separated_list1(space1, map(digit1, |n: &str| n.parse::<u32>().unwrap()))(text)?;
    let (text, _) = tuple((space1, char('|'), space1))(text)?;
    let (_, my_nums) =
        separated_list1(space1, map(digit1, |n: &str| n.parse::<u32>().unwrap()))(text)?;
    Ok(("", (win_nums, my_nums)))
}

fn my_winning_nums(win_nums: Vec<u32>, my_nums: Vec<u32>) -> Vec<u32> {
    let mut my_winning_nums = Vec::new();
    for num in my_nums {
        for win_num in win_nums.iter() {
            if &num == win_num {
                my_winning_nums.push(num);
            }
        }
    }
    my_winning_nums
}

pub fn part1(text: String) -> u32 {
    text.lines()
        .map(|line: &str| {
            let (_, (win_nums, my_nums)) = parse(line).unwrap();
            let my_winning_nums = my_winning_nums(win_nums, my_nums);
            match my_winning_nums.len() {
                0 => 0,
                n => 2u32.pow(n as u32 - 1),
            }
        })
        .sum()
}

pub fn part2(text: String) -> usize {
    let mut card2copies: HashMap<usize, usize> = HashMap::new();
    text.lines()
        .enumerate()
        .map(|(idx, line): (usize, &str)| {
            let (_, (win_nums, my_nums)) = parse(line).unwrap();
            (idx, my_winning_nums(win_nums, my_nums).len())
        })
        .map(|(idx, num_winning): (usize, usize)| {
            let num_copies = *card2copies.entry(idx).or_insert(1);
            for offset in 1..=num_winning {
                *card2copies.entry(idx + offset).or_insert(1) += num_copies
            }
            num_copies
        })
        .sum()
}
