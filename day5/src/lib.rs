use nom::bytes::complete::tag;
use nom::character::complete::*;
use nom::combinator::*;
use nom::multi::*;
use nom::sequence::*;
use nom::IResult;

#[derive(Debug)]
struct Rule {
    dst: u64,
    src: u64,
    len: u64,
}

impl Rule {
    fn map(&self, seed: u64) -> Option<u64> {
        if seed >= self.src && seed < self.src + self.len {
            Some(self.dst + seed - self.src)
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Map {
    rules: Vec<Rule>,
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<Map>,
}

fn parse_rule(input: &str) -> IResult<&str, Rule> {
    map(separated_list1(space1, digit1), |v: Vec<&str>| Rule {
        dst: v[0].parse().unwrap(),
        src: v[1].parse().unwrap(),
        len: v[2].parse().unwrap(),
    })(input)
}

fn parse_map(input: &str) -> IResult<&str, Map> {
    let (input, _) = terminated(separated_list1(char('-'), alpha1), tuple((space1, tag("map:"), multispace0)))(input)?;
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
        separated_list1(space1, map(digit1, |s: &str| s.parse::<u64>().unwrap())),
    )(input)?;
    let (_, maps) = preceded(multispace0, many1(parse_map))(input)?;
    Ok(("", Almanac { seeds, maps }))
}

pub fn part1(text: String) -> u64 {
    let (_, mut almanac) = parse_almanac(text.as_str()).unwrap();
    for seed in almanac.seeds.iter_mut() {
        for map in almanac.maps.iter() {
            for rule in map.rules.iter() {
                match rule.map(*seed) {
                    Some(s) => {
                        *seed = s;
                        break;
                    }
                    None => continue
                }
            }
        }
    }
    almanac.seeds.into_iter().min().unwrap()
}

fn parse_almanac2(input: &str) -> IResult<&str, Almanac> {
    let (input, seeds) = preceded(
        pair(tag("seeds:"), space1),
        separated_list1(space1, map(digit1, |s: &str| s.parse::<u64>().unwrap())),
    )(input)?;
    let (_, maps) = preceded(multispace0, many1(parse_map))(input)?;
    Ok(("", Almanac { seeds, maps }))
}

pub fn part2(text: String) -> i32 {
    todo!();
}
