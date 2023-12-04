use nom::IResult;
use nom::character::complete::*;
use nom::multi::*;
use nom::combinator::*;
use nom::sequence::*;
use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
struct Bag {
    red: u32,
    green: u32,
    blue: u32,
}

impl PartialOrd for Bag {
    fn partial_cmp(&self, other: &Bag) -> Option<Ordering> {
        if self.eq(other) {
            Some(Ordering::Equal)
        } else if self.red <= other.red && self.green <= other.green && self.blue <= other.blue {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Greater)
        }
    }
}

impl Bag {
    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<Bag>,
}

fn bag(text: &str) -> IResult<&str, Bag> {
    let mut bag = Bag {
        red: 0,
        green: 0,
        blue: 0,
    };
    let (text, _) = separated_list1(
        pair(char(','), space1), 
        map(
            separated_pair(u32, space1, alpha1),
            |(n, color): (u32, &str)| {
                match color {
                    "red" => bag.red = n,
                    "green" => bag.green = n,
                    "blue" => bag.blue = n,
                    _ => panic!(),
                }
            }
           )
        )(text)?;
    Ok((text, bag))
}

fn rounds(text: &str) -> IResult<&str, Vec<Bag>> {
    separated_list1(pair(char(';'), space1), bag)(text)
}

fn parse(text: &str) -> IResult<&str, Game> {
    let (text, _) = pair(alpha1, space1)(text)?;
    let (text, id) = u32(text)?;
    let (text, _) = pair(char(':'), space1)(text)?;
    let (_, rounds) = rounds(text)?;
    Ok((
            "",
            Game {
                id,
                rounds,
            }
       ))
}

pub fn part1(text: String) -> u32 {
    let config = Bag {
        red: 12,
        green: 13,
        blue: 14,
    };
    let mut sum = 0;
    for line in text.lines() {
        let (_, game) = parse(line).expect("Failed to parse.");
        let possible = game.rounds.iter().all(|round: &Bag| {
            round <= &config
        });
        if possible {
            sum += game.id;
        }
    }
    sum
}

pub fn part2(text: String) -> u32 {
    let mut sum = 0;
    for line in text.lines() {
        let (_, game) = parse(line).expect("Failed to parse.");
        let mut req_bag = Bag {red: 0, green: 0, blue: 0};
        for bag in game.rounds.iter() {
            if bag <= &req_bag {
                continue;
            }
            if bag.red > req_bag.red {
                req_bag.red = bag.red;
            }
            if bag.green > req_bag.green {
                req_bag.green = bag.green;
            }
            if bag.blue > req_bag.blue {
                req_bag.blue = bag.blue;
            }
        }
        sum += req_bag.power();
    }
    sum
}
