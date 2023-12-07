use std::cmp::Ordering;
use std::{collections::HashMap, iter::zip};

use nom::{
    character::complete::{alphanumeric1, digit1, multispace1, space1},
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

#[derive(PartialOrd, Ord, PartialEq, Eq)]
enum HandType {
    High,
    One,
    Two,
    Three,
    FullHouse,
    Four,
    Five,
}

#[derive(Debug)]
struct Player<'a> {
    hand: &'a str,
    bid: u32,
}

fn hand_type(hand: &str) -> HandType {
    let mut cards = HashMap::new();
    hand.chars().for_each(|c| {
        cards.entry(c).and_modify(|v| *v += 1).or_insert(1);
    });
    match cards.values().max().unwrap() {
        5 => HandType::Five,
        4 => HandType::Four,
        3 => match cards.keys().count() {
            2 => HandType::FullHouse,
            _ => HandType::Three,
        },
        2 => match cards.keys().count() {
            3 => HandType::Two,
            _ => HandType::One,
        },
        _ => HandType::High,
    }
}

fn hand_type_joker(hand: &str) -> HandType {
    let mut cards = HashMap::new();
    hand.chars().for_each(|c| {
        cards.entry(c).and_modify(|v| *v += 1).or_insert(1);
    });

    let num_jokers = cards.remove_entry(&'J').unwrap_or(('J', 0));
    match cards.values().max().unwrap_or(&0) + num_jokers.1 {
        5 => HandType::Five,
        4 => HandType::Four,
        3 => match cards.keys().count() {
            2 => HandType::FullHouse,
            _ => HandType::Three,
        },
        2 => match cards.keys().count() {
            3 => HandType::Two,
            _ => HandType::One,
        },
        _ => HandType::High,
    }
}

fn players(text: &str) -> IResult<&str, Vec<Player>> {
    separated_list1(
        multispace1,
        map(
            separated_pair(alphanumeric1, space1, digit1),
            |(hand, bid)| Player {
                hand,
                bid: bid.parse().unwrap(),
            },
        ),
    )(text)
}

pub fn part1(text: String) -> u32 {
    let (_, mut players) = players(text.as_str()).unwrap();
    players.sort_by(|p1, p2| {
        if hand_type(p1.hand) != hand_type(p2.hand) {
            return hand_type(p1.hand).cmp(&hand_type(p2.hand));
        }

        let strengths1 = p1
            .hand
            .chars()
            .map(|c| match c {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 11,
                'T' => 10,
                n => n.to_digit(10).unwrap(),
            })
            .collect::<Vec<u32>>();
        let strengths2 = p2
            .hand
            .chars()
            .map(|c| match c {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 11,
                'T' => 10,
                n => n.to_digit(10).unwrap(),
            })
            .collect::<Vec<u32>>();

        for (strength1, strength2) in zip(strengths1, strengths2) {
            if strength1 != strength2 {
                return strength1.cmp(&strength2);
            }
        }
        Ordering::Equal
    });
    players
        .iter()
        .enumerate()
        .map(|(idx, p)| (idx as u32 + 1) * p.bid)
        .sum()
}

pub fn part2(text: String) -> u32 {
    let (_, mut players) = players(text.as_str()).unwrap();
    players.sort_by(|p1, p2| {
        if hand_type_joker(p1.hand) != hand_type_joker(p2.hand) {
            return hand_type_joker(p1.hand).cmp(&hand_type_joker(p2.hand));
        }

        let strengths1 = p1
            .hand
            .chars()
            .map(|c| match c {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 1,
                'T' => 10,
                n => n.to_digit(10).unwrap(),
            })
            .collect::<Vec<u32>>();
        let strengths2 = p2
            .hand
            .chars()
            .map(|c| match c {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 1,
                'T' => 10,
                n => n.to_digit(10).unwrap(),
            })
            .collect::<Vec<u32>>();

        for (strength1, strength2) in zip(strengths1, strengths2) {
            if strength1 != strength2 {
                return strength1.cmp(&strength2);
            }
        }
        Ordering::Equal
    });
    players
        .iter()
        .enumerate()
        .map(|(idx, p)| (idx as u32 + 1) * p.bid)
        .sum()
}
