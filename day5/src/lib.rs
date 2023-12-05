use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::*;
use nom::combinator::*;
use nom::multi::*;
use nom::sequence::*;
use nom::IResult;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Range {
    src: u64,
    len: u64,
}

impl Range {
    /// Partition into multiple ranges that do not cross the given range boundaries
    fn partition(&self, other: Range) -> Option<Vec<Range>> {
        if self.src + self.len < other.src {
            return None;
        }
        if self.src >= other.src + other.len {
            return None;
        }

        let mut new_ranges = Vec::new();
        if self.src < other.src {
            new_ranges.push(Range {
                src: self.src,
                len: other.src - self.src,
            });

            if self.src + self.len <= other.src + other.len {
                new_ranges.push(Range {
                    src: other.src,
                    len: self.src + self.len - other.src,
                })
            } else {
                new_ranges.push(other);
                new_ranges.push(Range {
                    src: other.src + other.len,
                    len: self.src + self.len - other.src - other.len,
                })
            }
        } else if self.src + self.len <= other.src + other.len {
            new_ranges.push(*self);
        } else {
            new_ranges.push(Range {
                src: self.src,
                len: other.src + other.len - self.src,
            });
            new_ranges.push(Range {
                src: other.src + other.len,
                len: self.src + self.len - other.src - other.len,
            });
        }
        Some(new_ranges)
    }
}

#[derive(Debug)]
struct Rule {
    dst: u64,
    range: Range,
}

impl Rule {
    fn map(&self, range: Range) -> Option<Range> {
        // Assume input range does not cross the rule range boundaries
        if range.src < self.range.src || range.src > self.range.src + self.range.len {
            None
        } else {
            Some(Range {
                src: self.dst + range.src - self.range.src,
                len: range.len,
            })
        }
    }
}

#[derive(Debug)]
struct Map {
    rules: Vec<Rule>,
}

impl Map {
    /// Partition the given range with the rules in the map AND map based on the rules
    fn partition_map(&self, range: Range) -> Vec<Range> {
        let mut parted_ranges = Vec::new();
        for rule in self.rules.iter() {
            match range.partition(rule.range) {
                Some(ranges) => {
                    parted_ranges.extend(ranges.into_iter());
                }
                None => continue,
            }
        }
        if parted_ranges.is_empty() {
            parted_ranges.push(range);
        }
        let mut mapped_ranges = Vec::new();
        for range in parted_ranges.into_iter().unique() {
            let mut unmapped = true;
            for rule in self.rules.iter() {
                match rule.map(range) {
                    Some(r) => {
                        mapped_ranges.push(r);
                        unmapped = false;
                        break;
                    }
                    None => continue,
                }
            }
            if unmapped {
                mapped_ranges.push(range);
            }
        }
        mapped_ranges
    }
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<Range>,
    maps: Vec<Map>,
}

fn parse_rule(input: &str) -> IResult<&str, Rule> {
    map(separated_list1(space1, digit1), |v: Vec<&str>| Rule {
        dst: v[0].parse().unwrap(),
        range: Range {
            src: v[1].parse().unwrap(),
            len: v[2].parse().unwrap(),
        },
    })(input)
}

fn parse_map(input: &str) -> IResult<&str, Map> {
    let (input, _) = terminated(
        separated_list1(char('-'), alpha1),
        tuple((space1, tag("map:"), multispace0)),
    )(input)?;
    let (input, map) = fold_many1(
        terminated(parse_rule, multispace0),
        || Map { rules: Vec::new() },
        |mut acc, r| {
            acc.rules.push(r);
            acc
        },
    )(input)?;
    Ok((input, map))
}

fn parse_almanac(input: &str) -> IResult<&str, Almanac> {
    let (input, seeds) = preceded(
        pair(tag("seeds:"), space1),
        separated_list1(
            space1,
            map(digit1, |s: &str| Range {
                src: s.parse::<u64>().unwrap(),
                len: 1,
            }),
        ),
    )(input)?;
    let (_, maps) = preceded(multispace0, many1(parse_map))(input)?;
    Ok(("", Almanac { seeds, maps }))
}

fn find_min_seed(almanac: Almanac) -> u64 {
    let mut seeds = almanac.seeds;
    let mut mapped_seeds = Vec::new();
    for map in almanac.maps.iter() {
        for seed in seeds.into_iter() {
            mapped_seeds.extend(map.partition_map(seed).iter());
        }
        seeds = mapped_seeds;
        mapped_seeds = Vec::new();
    }
    seeds.into_iter().map(|s: Range| s.src).min().unwrap()
}

pub fn part1(text: String) -> u64 {
    let (_, almanac) = parse_almanac(text.as_str()).unwrap();
    find_min_seed(almanac)
}

fn parse_almanac2(input: &str) -> IResult<&str, Almanac> {
    let (input, seeds) = preceded(
        pair(tag("seeds:"), space1),
        separated_list1(
            space1,
            map(
                separated_pair(digit1, space1, digit1),
                |(s1, s2): (&str, &str)| Range {
                    src: s1.parse::<u64>().unwrap(),
                    len: s2.parse::<u64>().unwrap(),
                },
            ),
        ),
    )(input)?;
    let (_, maps) = preceded(multispace0, many1(parse_map))(input)?;
    Ok(("", Almanac { seeds, maps }))
}

pub fn part2(text: String) -> u64 {
    let (_, almanac) = parse_almanac2(text.as_str()).unwrap();
    find_min_seed(almanac)
}
