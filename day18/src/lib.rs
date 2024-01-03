use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use glam::I64Vec2;
use nom::{
    character::complete::{alphanumeric1, char, digit1, line_ending, one_of, space1},
    combinator::map,
    error::Error,
    multi::separated_list1,
    sequence::{delimited, preceded, tuple},
    IResult,
};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Hole {
    color: String,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Grid {
    min: I64Vec2,
    max: I64Vec2,
    blocks: HashMap<I64Vec2, Hole>,
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in self.min.y..self.max.y {
            for x in self.min.x..self.max.x {
                let loc = I64Vec2::new(x, y);
                if self.blocks.contains_key(&loc) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        writeln!(f)
    }
}

fn get_grid(text: &str) -> IResult<&str, Grid> {
    map(
        separated_list1(
            line_ending::<&str, Error<_>>,
            tuple((
                one_of("UDLR"),
                space1,
                digit1,
                space1,
                delimited(char('('), preceded(char('#'), alphanumeric1), char(')')),
            )),
        ),
        |lines| {
            let mut blocks = HashMap::new();
            let mut cur = I64Vec2::new(0, 0);
            for (dir, _, len, _, color) in lines {
                for _ in 0..len.parse().unwrap() {
                    match dir {
                        'U' => {
                            cur -= I64Vec2::new(0, 1);
                            blocks.insert(
                                cur,
                                Hole {
                                    color: color.to_string(),
                                },
                            );
                        }
                        'D' => {
                            cur += I64Vec2::new(0, 1);
                            blocks.insert(
                                cur,
                                Hole {
                                    color: color.to_string(),
                                },
                            );
                        }
                        'L' => {
                            cur -= I64Vec2::new(1, 0);
                            blocks.insert(
                                cur,
                                Hole {
                                    color: color.to_string(),
                                },
                            );
                        }
                        'R' => {
                            cur += I64Vec2::new(1, 0);
                            blocks.insert(
                                cur,
                                Hole {
                                    color: color.to_string(),
                                },
                            );
                        }
                        _ => panic!("Invalid direction"),
                    }
                }
            }
            let bounds = blocks.iter().fold(
                (I64Vec2::new(0, 0), I64Vec2::new(0, 0)),
                |(acc_min, acc_max), (loc, _)| {
                    let x = acc_min.x.min(loc.x);
                    let y = acc_min.y.min(loc.y);
                    let min = I64Vec2::new(x, y);
                    let x = acc_max.x.max(loc.x);
                    let y = acc_max.y.max(loc.y);
                    let max = I64Vec2::new(x, y);
                    (min, max)
                },
            );
            Grid {
                min: bounds.0,
                max: bounds.1 + I64Vec2::new(1, 1),
                blocks,
            }
        },
    )(text)
}

fn count_interior(grid: &Grid) -> usize {
    let mut exterior = HashSet::new();
    let mut exterior_boundary = Vec::new();

    // Mark boundary as exterior
    for x in grid.min.x..grid.max.x {
        let block = I64Vec2::new(x, grid.min.y);
        if !grid.blocks.contains_key(&block) {
            exterior_boundary.push(block);
        }
        let block = I64Vec2::new(x, grid.max.y - 1);
        if !grid.blocks.contains_key(&block) {
            exterior_boundary.push(block);
        }
    }
    for y in grid.min.y..grid.max.y {
        let block = I64Vec2::new(grid.min.x, y);
        if !grid.blocks.contains_key(&block) {
            exterior_boundary.push(block);
        }
        let block = I64Vec2::new(grid.max.x - 1, y);
        if !grid.blocks.contains_key(&block) {
            exterior_boundary.push(block);
        }
    }

    // Spread the exterior
    // Use a quad tree approach to exponentially break the space into smaller pieces
    // Keep splitting the space until it doesn't contain any of the trenches (check with boundaries)
    // If the smaller space is next to a space that has already been marked as exterior,
    // then it is exterior
    while let Some(block) = exterior_boundary.pop() {
        if grid.blocks.contains_key(&block) {
            continue;
        }
        exterior.insert(block);
        let adj_blocks = [
            block + I64Vec2::new(1, 0),
            block - I64Vec2::new(1, 0),
            block + I64Vec2::new(0, 1),
            block - I64Vec2::new(0, 1),
        ];
        for adj_block in adj_blocks.into_iter() {
            if !exterior.contains(&adj_block)
                && adj_block.x >= grid.min.x
                && adj_block.y >= grid.min.y
                && adj_block.x < grid.max.x
                && adj_block.y < grid.max.y
            {
                exterior_boundary.push(adj_block);
            }
        }
    }

    // Interior count is the total blocks in bounds - exterior
    let bounding_area = (grid.max.x - grid.min.x) * (grid.max.y - grid.min.y);
    bounding_area as usize - exterior.len()
}

pub fn part1(text: String) -> usize {
    let (_, grid) = get_grid(text.as_str()).unwrap();
    count_interior(&grid)
}

enum Direction {
    North,
    South,
    East,
    West,
}

fn get_grid_by_color(text: &str) -> IResult<&str, Vec<(Direction, i64)>> {
    map(
        separated_list1(
            line_ending::<&str, Error<_>>,
            tuple((
                one_of("UDLR"),
                space1,
                digit1,
                space1,
                delimited(char('('), preceded(char('#'), alphanumeric1), char(')')),
            )),
        ),
        |lines| {
            let mut segments = Vec::new();
            for (_, _, _, _, color) in lines {
                let len = i64::from_str_radix(&color[..5], 16).unwrap();
                let dir = i64::from_str_radix(&color[5..], 16).unwrap();
                segments.push((
                    match dir {
                        3 => Direction::North,
                        1 => Direction::South,
                        0 => Direction::East,
                        2 => Direction::West,
                        _ => panic!("Invalid direction"),
                    },
                    len,
                ));
            }
            segments
        },
    )(text)
}

/// Calculate the interior of a polygon using the Triangle Form of the Shoelace Formula
/// This version counts the boundary as part of the interior
/// Note: if boundary is not in the interior, subtract len rather than add it, still +1 at the end
fn shoelace(segments: &[(Direction, i64)]) -> i64 {
    segments
        .iter()
        .fold((I64Vec2::new(0, 0), 0), |(cur, area), (dir, len)| {
            let v1 = cur;
            let v2 = cur
                + match dir {
                    Direction::North => I64Vec2::new(0, -*len),
                    Direction::South => I64Vec2::new(0, *len),
                    Direction::East => I64Vec2::new(*len, 0),
                    Direction::West => I64Vec2::new(-*len, 0),
                };
            dbg!(v1);
            dbg!(v2);
            (v2, area + v1.x * v2.y - v2.x * v1.y + len)
        })
        .1
        / 2
        + 1
}

pub fn part2(text: String) -> i64 {
    let (_, grid) = get_grid_by_color(text.as_str()).unwrap();
    shoelace(&grid)
}
