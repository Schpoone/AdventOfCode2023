use std::iter::zip;

use nom::{
    bytes::complete::take_until,
    character::complete::{anychar, digit1, space1},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};

fn parse(text: &str) -> IResult<&str, (Vec<u32>, Vec<u32>)> {
    let (text, times) = preceded(
        tuple((take_until(":"), anychar, space1)),
        separated_list1(space1, map(digit1, |s: &str| s.parse::<u32>().unwrap())),
    )(text)?;
    let (text, distances) = preceded(
        tuple((take_until(":"), anychar, space1)),
        separated_list1(space1, map(digit1, |s: &str| s.parse::<u32>().unwrap())),
    )(text)?;
    Ok((text, (times, distances)))
}

/// Boat distance if button held for button_secs and race is total_secs
fn dist(button_secs: u32, total_secs: u32) -> u32 {
    (total_secs - button_secs) * button_secs
}

pub fn part1(text: String) -> u32 {
    let (_, (times, distances)) = parse(text.as_str()).unwrap();
    zip(times, distances)
        .map(|(time, distance)| {
            let mut min = 0;
            while dist(min, time) <= distance {
                min += 1;
            }
            let mut max = time;
            while dist(max, time) <= distance {
                max -= 1;
            }
            max - min + 1
        })
        .product()
}

fn parse2(text: &str) -> IResult<&str, (u64, u64)> {
    let (text, times) = preceded(
        tuple((take_until(":"), anychar, space1)),
        separated_list1(space1, digit1),
    )(text)?;
    let (text, distances) = preceded(
        tuple((take_until(":"), anychar, space1)),
        separated_list1(space1, digit1),
    )(text)?;
    Ok((
        text,
        (
            times.concat().parse().unwrap(),
            distances.concat().parse().unwrap(),
        ),
    ))
}

/// Boat distance if button held for button_secs and race is total_secs
fn dist2(button_secs: u64, total_secs: u64) -> u64 {
    (total_secs - button_secs) * button_secs
}

pub fn part2(text: String) -> u64 {
    let (_, (time, distance)) = parse2(text.as_str()).unwrap();
    let min;
    let mut lower = 0;
    let mut upper = time / 2;
    while upper - lower > 1 {
        let mid = (lower + upper) / 2;
        if dist2(mid, time) <= distance {
            lower = mid;
        } else {
            upper = mid;
        }
    }
    if dist2(lower, time) <= distance && dist2(upper, time) > distance {
        min = upper;
    } else if dist2(lower, time) > distance && dist2(upper, time) <= distance {
        min = lower;
    } else {
        panic!("Binary search failed");
    }
    let max;
    let mut lower = time / 2 + 1;
    let mut upper = time;
    while upper - lower > 1 {
        let mid = (lower + upper) / 2;
        if dist2(mid, time) <= distance {
            upper = mid;
        } else {
            lower = mid;
        }
    }
    if dist2(lower, time) <= distance && dist2(upper, time) > distance {
        max = upper;
    } else if dist2(lower, time) > distance && dist2(upper, time) <= distance {
        max = lower;
    } else {
        panic!("Binary search failed");
    }
    max - min + 1
}
