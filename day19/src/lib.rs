use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, char, digit1, line_ending, multispace1, one_of},
    combinator::map,
    error::Error,
    multi::{fold_many1, separated_list1},
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};

#[derive(Debug)]
enum Destination<'a> {
    A,
    R,
    Workflow(&'a str),
}

#[derive(Debug)]
struct Rule<'a> {
    condition: Option<(char, char, u32)>,
    dest: Destination<'a>,
}

impl Rule<'_> {
    fn applies(&self, part: &Part) -> bool {
        if let Some((var, op, num)) = self.condition {
            match var {
                'x' => match op {
                    '<' => part.x < num,
                    '>' => part.x > num,
                    _ => panic!("Failed to parse condition op"),
                },
                'm' => match op {
                    '<' => part.m < num,
                    '>' => part.m > num,
                    _ => panic!("Failed to parse condition op"),
                },
                'a' => match op {
                    '<' => part.a < num,
                    '>' => part.a > num,
                    _ => panic!("Failed to parse condition op"),
                },
                's' => match op {
                    '<' => part.s < num,
                    '>' => part.s > num,
                    _ => panic!("Failed to parse condition op"),
                },
                _ => panic!("Failed to parse condition variable"),
            }
        } else {
            true
        }
    }

    /// Checks if this rule applies to the given range
    ///
    /// Returns a pair where the first range is the part of the given range that
    /// satisfies the rule condition and the second range is the part that doesn't
    fn applies_to_range(&self, range: &PartRange) -> (Option<PartRange>, Option<PartRange>) {
        if let Some((var, op, num)) = self.condition {
            match var {
                'x' => match op {
                    '<' => match (num >= range.x.min, num < range.x.max) {
                        (true, false) => (Some(*range), None),
                        (false, true) => (None, Some(*range)),
                        _ => (
                            Some(PartRange {
                                x: Range::new(range.x.min, num),
                                m: range.m,
                                a: range.a,
                                s: range.s,
                            }),
                            Some(PartRange {
                                x: Range::new(num, range.x.max),
                                m: range.m,
                                a: range.a,
                                s: range.s,
                            }),
                        ),
                    },
                    '>' => match (num >= range.x.min, num < range.x.max) {
                        (false, true) => (Some(*range), None),
                        (true, false) => (None, Some(*range)),
                        _ => (
                            Some(PartRange {
                                x: Range::new(num + 1, range.x.max),
                                m: range.m,
                                a: range.a,
                                s: range.s,
                            }),
                            Some(PartRange {
                                x: Range::new(range.x.min, num + 1),
                                m: range.m,
                                a: range.a,
                                s: range.s,
                            }),
                        ),
                    },
                    _ => panic!("Failed to parse condition op"),
                },
                'm' => match op {
                    '<' => match (num >= range.m.min, num < range.m.max) {
                        (true, false) => (Some(*range), None),
                        (false, true) => (None, Some(*range)),
                        _ => (
                            Some(PartRange {
                                x: range.x,
                                m: Range::new(range.m.min, num),
                                a: range.a,
                                s: range.s,
                            }),
                            Some(PartRange {
                                x: range.x,
                                m: Range::new(num, range.m.max),
                                a: range.a,
                                s: range.s,
                            }),
                        ),
                    },
                    '>' => match (num >= range.m.min, num < range.m.max) {
                        (false, true) => (Some(*range), None),
                        (true, false) => (None, Some(*range)),
                        _ => (
                            Some(PartRange {
                                x: range.x,
                                m: Range::new(num + 1, range.m.max),
                                a: range.a,
                                s: range.s,
                            }),
                            Some(PartRange {
                                x: range.x,
                                m: Range::new(range.m.min, num + 1),
                                a: range.a,
                                s: range.s,
                            }),
                        ),
                    },
                    _ => panic!("Failed to parse condition op"),
                },
                'a' => match op {
                    '<' => match (num >= range.a.min, num < range.a.max) {
                        (true, false) => (Some(*range), None),
                        (false, true) => (None, Some(*range)),
                        _ => (
                            Some(PartRange {
                                x: range.x,
                                m: range.m,
                                a: Range::new(range.a.min, num),
                                s: range.s,
                            }),
                            Some(PartRange {
                                x: range.x,
                                m: range.m,
                                a: Range::new(num, range.a.max),
                                s: range.s,
                            }),
                        ),
                    },
                    '>' => match (num >= range.a.min, num < range.a.max) {
                        (false, true) => (Some(*range), None),
                        (true, false) => (None, Some(*range)),
                        _ => (
                            Some(PartRange {
                                x: range.x,
                                m: range.m,
                                a: Range::new(num + 1, range.a.max),
                                s: range.s,
                            }),
                            Some(PartRange {
                                x: range.x,
                                m: range.m,
                                a: Range::new(range.a.min, num + 1),
                                s: range.s,
                            }),
                        ),
                    },
                    _ => panic!("Failed to parse condition op"),
                },
                's' => match op {
                    '<' => match (num >= range.s.min, num < range.s.max) {
                        (true, false) => (Some(*range), None),
                        (false, true) => (None, Some(*range)),
                        _ => (
                            Some(PartRange {
                                x: range.x,
                                m: range.m,
                                a: range.a,
                                s: Range::new(range.s.min, num),
                            }),
                            Some(PartRange {
                                x: range.x,
                                m: range.m,
                                a: range.a,
                                s: Range::new(num, range.s.max),
                            }),
                        ),
                    },
                    '>' => match (num >= range.s.min, num < range.s.max) {
                        (false, true) => (Some(*range), None),
                        (true, false) => (None, Some(*range)),
                        _ => (
                            Some(PartRange {
                                x: range.x,
                                m: range.m,
                                a: range.a,
                                s: Range::new(num + 1, range.s.max),
                            }),
                            Some(PartRange {
                                x: range.x,
                                m: range.m,
                                a: range.a,
                                s: Range::new(range.s.min, num + 1),
                            }),
                        ),
                    },
                    _ => panic!("Failed to parse condition op"),
                },
                _ => panic!("Failed to parse condition variable"),
            }
        } else {
            (Some(*range), None)
        }
    }
}

#[derive(Debug)]
struct Workflow<'a> {
    name: &'a str,
    rules: Vec<Rule<'a>>,
}

#[derive(Debug)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl Part {
    fn total_ratings(&self) -> u32 {
        self.x + self.m + self.a + self.s
    }
}

fn parse_rule(text: &str) -> IResult<&str, Rule> {
    alt((
        map(
            tuple((
                one_of("xmas"),
                one_of("<>"),
                digit1,
                preceded(char(':'), alpha1),
            )),
            |(var, op, num, dest)| {
                let dest = match dest {
                    "A" => Destination::A,
                    "R" => Destination::R,
                    workflow => Destination::Workflow(workflow),
                };
                Rule {
                    condition: Some((var, op, num.parse().unwrap())),
                    dest,
                }
            },
        ),
        map(alpha1, |dest| {
            let dest = match dest {
                "A" => Destination::A,
                "R" => Destination::R,
                workflow => Destination::Workflow(workflow),
            };
            Rule {
                condition: None,
                dest,
            }
        }),
    ))(text)
}

fn parse_workflow(text: &str) -> IResult<&str, Workflow> {
    map(
        tuple((
            alpha1,
            delimited(char('{'), separated_list1(char(','), parse_rule), char('}')),
        )),
        |(name, rules)| Workflow { name, rules },
    )(text)
}

fn parse_part(text: &str) -> IResult<&str, Part> {
    map(
        delimited(
            char::<&str, Error<&str>>('{'),
            tuple((
                delimited(tag("x="), digit1, char(',')),
                delimited(tag("m="), digit1, char(',')),
                delimited(tag("a="), digit1, char(',')),
                preceded(tag("s="), digit1),
            )),
            char('}'),
        ),
        |(x, m, a, s)| Part {
            x: x.parse::<u32>().unwrap(),
            m: m.parse::<u32>().unwrap(),
            a: a.parse::<u32>().unwrap(),
            s: s.parse::<u32>().unwrap(),
        },
    )(text)
}

fn parse_input(text: &str) -> IResult<&str, (HashMap<&str, Workflow>, Vec<Part>)> {
    let (text, workflows) = fold_many1(
        terminated(parse_workflow, line_ending),
        HashMap::new,
        |mut acc, item| {
            acc.insert(item.name, item);
            acc
        },
    )(text)?;
    let (text, parts) = preceded(multispace1, separated_list1(multispace1, parse_part))(text)?;
    Ok((text, (workflows, parts)))
}

pub fn part1(text: String) -> u32 {
    let (_, (workflows, parts)) = parse_input(text.as_str()).unwrap();
    parts
        .into_iter()
        .filter_map(|part| {
            let mut workflow = "in";
            loop {
                for rule in workflows.get(workflow).unwrap().rules.iter() {
                    if rule.applies(&part) {
                        match rule.dest {
                            Destination::A => return Some(part.total_ratings()),
                            Destination::R => return None,
                            Destination::Workflow(w) => {
                                workflow = w;
                                break;
                            }
                        }
                    }
                }
            }
        })
        .sum()
}

#[derive(Debug, Clone, Copy)]
struct Range {
    min: u32,
    max: u32,
}

#[derive(Debug, Clone, Copy)]
struct PartRange {
    x: Range,
    m: Range,
    a: Range,
    s: Range,
}

impl Range {
    fn new(min: u32, max: u32) -> Self {
        Self { min, max }
    }
}

impl PartRange {
    fn total_ratings(&self) -> u64 {
        (self.x.max - self.x.min) as u64
            * (self.m.max - self.m.min) as u64
            * (self.a.max - self.a.min) as u64
            * (self.s.max - self.s.min) as u64
    }
}

pub fn part2(text: String) -> u64 {
    let (_, (workflows, _)) = parse_input(text.as_str()).unwrap();
    let mut combinations = 0u64;
    let mut part_ranges = Vec::new();
    part_ranges.push((
        "in",
        PartRange {
            x: Range::new(1, 4001),
            m: Range::new(1, 4001),
            a: Range::new(1, 4001),
            s: Range::new(1, 4001),
        },
    ));
    while let Some((workflow, mut part_range)) = part_ranges.pop() {
        for rule in workflows.get(workflow).unwrap().rules.iter() {
            let (true_range, false_range) = rule.applies_to_range(&part_range);
            if let Some(true_range) = true_range {
                match rule.dest {
                    Destination::A => combinations += true_range.total_ratings(),
                    Destination::R => {}
                    Destination::Workflow(w) => part_ranges.push((w, true_range)),
                }
            }
            if let Some(false_range) = false_range {
                part_range = false_range;
            } else {
                break;
            }
        }
    }
    combinations
}
