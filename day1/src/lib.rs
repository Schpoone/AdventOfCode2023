use nom::character::complete::*;
use nom::{branch::alt, bytes::complete::*, combinator::*, multi::*, sequence::*, IResult};
use std::str;

fn parse1(text: &str) -> IResult<&str, i32> {
    let (_, digits) = many1(preceded(alpha0, anychar))(text)?;
    let num_str = String::from_iter([digits[0], digits[digits.len() - 1]].iter());
    Ok(("", num_str.parse::<i32>().unwrap()))
}

fn map_spelling(num_str: &str) -> char {
    match num_str {
        "zero" => '0',
        "one" => '1',
        "two" => '2',
        "three" => '3',
        "four" => '4',
        "five" => '5',
        "six" => '6',
        "seven" => '7',
        "eight" => '8',
        "nine" => '9',
        _ => panic!(),
    }
}

fn my_digit(text: &str) -> IResult<&str, char> {
    terminated(
        peek(alt((
            one_of("0123456789"),
            map(tag("zero"), map_spelling),
            map(tag("one"), map_spelling),
            map(tag("two"), map_spelling),
            map(tag("three"), map_spelling),
            map(tag("four"), map_spelling),
            map(tag("five"), map_spelling),
            map(tag("six"), map_spelling),
            map(tag("seven"), map_spelling),
            map(tag("eight"), map_spelling),
            map(tag("nine"), map_spelling),
        ))),
        anychar,
    )(text)
}

fn parse2(text: &str) -> IResult<&str, i32> {
    let (_, digits) = many0(preceded(many0(pair(not(my_digit), anychar)), my_digit))(text)?;
    let num_str = String::from_iter([digits[0], digits[digits.len() - 1]].iter());
    Ok(("", num_str.parse::<i32>().unwrap()))
}

pub fn part1(text: String) -> i32 {
    let mut sum = 0;
    for line in text.lines() {
        let (_, num) = parse1(line.trim()).unwrap();
        sum += num;
    }
    sum
}

pub fn part2(text: String) -> i32 {
    let mut sum = 0;
    for line in text.lines() {
        let (_, num) = parse2(line.trim()).unwrap();
        sum += num;
    }
    sum
}
