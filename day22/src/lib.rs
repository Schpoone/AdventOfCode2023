use std::collections::{HashMap, HashSet};

use glam::{IVec2, IVec3};
use itertools::Itertools;
use nom::{
    character::complete::{char, digit1, line_ending},
    combinator::map,
    multi::separated_list1,
    sequence::{separated_pair, tuple},
    IResult,
};

#[derive(Debug)]
struct Brick {
    start: IVec3,
    end: IVec3,
}

impl Brick {
    fn get_cubes(&self) -> Vec<IVec3> {
        if self.start.x < self.end.x {
            (self.start.x..=self.end.x)
                .map(|x| IVec3::new(x, self.start.y, self.start.z))
                .collect::<Vec<IVec3>>()
        } else if self.start.y < self.end.y {
            (self.start.y..=self.end.y)
                .map(|y| IVec3::new(self.start.x, y, self.start.z))
                .collect::<Vec<IVec3>>()
        } else if self.start.z < self.end.z {
            (self.start.z..=self.end.z)
                .map(|z| IVec3::new(self.start.x, self.start.y, z))
                .collect::<Vec<IVec3>>()
        } else {
            vec![self.start]
        }
    }
}

fn parse_vec(text: &str) -> IResult<&str, IVec3> {
    map(
        tuple((digit1, char(','), digit1, char(','), digit1)),
        |(x, _, y, _, z): (&str, char, &str, char, &str)| {
            IVec3::new(
                x.parse::<i32>().unwrap(),
                y.parse::<i32>().unwrap(),
                z.parse::<i32>().unwrap(),
            )
        },
    )(text)
}

fn parse_bricks(text: &str) -> IResult<&str, Vec<Brick>> {
    separated_list1(
        line_ending,
        map(
            separated_pair(parse_vec, char('~'), parse_vec),
            |(v1, v2)| {
                if v1.x <= v2.x || v1.y <= v2.y || v1.z <= v2.z {
                    Brick { start: v1, end: v2 }
                } else {
                    Brick { start: v2, end: v1 }
                }
            },
        ),
    )(text)
}

pub fn part1(text: String) -> usize {
    let (_, bricks) = parse_bricks(text.as_str()).unwrap();
    let mut height_map: HashMap<IVec2, (i32, Option<usize>)> = HashMap::new();
    let mut support_map: HashMap<usize, HashSet<usize>> = HashMap::new();
    for (idx, brick) in bricks
        .into_iter()
        .sorted_by_key(|brick| brick.start.z)
        .enumerate()
    {
        let cubes = brick.get_cubes();
        let landing_height = cubes
            .iter()
            .map(|cube| {
                height_map
                    .get(&IVec2::new(cube.x, cube.y))
                    .unwrap_or(&(0, None))
                    .0
            })
            .max()
            .unwrap();
        let mut supported_by = HashSet::new();
        for cube in cubes {
            let height_map_entry = height_map
                .entry(IVec2::new(cube.x, cube.y))
                .or_insert((0, None));
            if height_map_entry.0 == landing_height {
                if let Some(other_idx) = height_map_entry.1 {
                    if other_idx != idx {
                        supported_by.insert(other_idx);
                    }
                }
            }
            let brick_height = (brick.end - brick.start).z;
            *height_map_entry = (landing_height + brick_height + 1, Some(idx));
        }
        support_map.insert(idx, supported_by);
    }
    support_map.len()
        - support_map
            .into_iter()
            .filter_map(|(_, supporters)| {
                if supporters.len() == 1 {
                    Some(supporters.into_iter().next().unwrap())
                } else {
                    None
                }
            })
            .unique()
            .count()
}

pub fn part2(text: String) -> usize {
    let (_, bricks) = parse_bricks(text.as_str()).unwrap();
    let mut height_map: HashMap<IVec2, (i32, Option<usize>)> = HashMap::new();
    let mut support_map: HashMap<usize, HashSet<usize>> = HashMap::new();
    for (idx, brick) in bricks
        .into_iter()
        .sorted_by_key(|brick| brick.start.z)
        .enumerate()
    {
        let cubes = brick.get_cubes();
        let landing_height = cubes
            .iter()
            .map(|cube| {
                height_map
                    .get(&IVec2::new(cube.x, cube.y))
                    .unwrap_or(&(0, None))
                    .0
            })
            .max()
            .unwrap();
        let mut supported_by = HashSet::new();
        for cube in cubes {
            let height_map_entry = height_map
                .entry(IVec2::new(cube.x, cube.y))
                .or_insert((0, None));
            if height_map_entry.0 == landing_height {
                if let Some(other_idx) = height_map_entry.1 {
                    if other_idx != idx {
                        supported_by.insert(other_idx);
                    }
                }
            }
            let brick_height = (brick.end - brick.start).z;
            *height_map_entry = (landing_height + brick_height + 1, Some(idx));
        }
        support_map.insert(idx, supported_by);
    }
    let unsafe_bricks = support_map
        .iter()
        .filter_map(|(_, supporters)| {
            if supporters.len() == 1 {
                Some(*supporters.iter().next().unwrap())
            } else {
                None
            }
        })
        .collect::<HashSet<usize>>();
    unsafe_bricks
        .into_iter()
        .map(|disintegrated_brick_idx| {
            let mut fallen_bricks = HashSet::new();
            fallen_bricks.insert(disintegrated_brick_idx);
            loop {
                let num_fallen_bricks = fallen_bricks.len();
                for (brick_idx, supporters) in support_map.iter() {
                    if !supporters.is_empty() && supporters.difference(&fallen_bricks).count() == 0
                    {
                        fallen_bricks.insert(*brick_idx);
                    }
                }
                if fallen_bricks.len() == num_fallen_bricks {
                    break;
                }
            }
            fallen_bricks.len() - 1
        })
        .sum()
}
