use std::collections::HashMap;

use nom::{
    branch::alt,
    character::complete::{alpha1, char, digit0},
    combinator::map,
    multi::separated_list1,
    sequence::pair,
    IResult,
};

fn custom_hash(input: &str) -> u32 {
    input
        .chars()
        .fold(0, |acc, c| ((acc + (c as u32)) * 17) % 256)
}

pub fn part1(text: String) -> u32 {
    text.trim().split(',').map(custom_hash).sum()
}

#[derive(Debug)]
enum LensOp<'a> {
    Remove(Lens<'a>),
    Add(Lens<'a>),
}

#[derive(Debug, Clone)]
struct Lens<'a> {
    label: &'a str,
    focal_len: u32,
}

fn parse_ops(text: &str) -> IResult<&str, Vec<LensOp>> {
    separated_list1(
        char(','),
        map(
            pair(alpha1, pair(alt((char('-'), char('='))), digit0)),
            |(label, (op, lens))| match op {
                '-' => LensOp::Remove(Lens {
                    label,
                    focal_len: 0,
                }),
                '=' => LensOp::Add(Lens {
                    label,
                    focal_len: lens.chars().last().unwrap().to_digit(10).unwrap(),
                }),
                _ => panic!("failed to parse"),
            },
        ),
    )(text)
}

pub fn part2(text: String) -> u32 {
    let (_, ops) = parse_ops(text.as_str()).unwrap();
    let mut boxes: HashMap<u32, Vec<Lens>> = HashMap::new();
    for op in ops {
        match op {
            LensOp::Remove(lens) => {
                let box_idx = custom_hash(lens.label);
                boxes.entry(box_idx).and_modify(|b| {
                    b.retain(|l| l.label != lens.label);
                }).or_default();
            }
            LensOp::Add(lens) => {
                let box_idx = custom_hash(lens.label);
                boxes.entry(box_idx).and_modify(|b| {
                    let lens_idx = b.iter().position(|l| l.label == lens.label);
                    match lens_idx {
                        Some(idx) => b[idx] = lens.clone(),
                        None => b.push(lens.clone()),
                    }
                }).or_insert(vec![lens.clone()]);
            }
        }
    }
    boxes.into_iter().map(|(box_idx, lenses)| {
        lenses.iter().enumerate().map(|(i, lens)| {
            (box_idx + 1) * (i as u32 + 1) * lens.focal_len
        }).sum::<u32>()
    }).sum()
}
