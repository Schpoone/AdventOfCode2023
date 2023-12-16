use std::collections::{HashMap, HashSet};

use glam::IVec2;

#[derive(Debug, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, PartialEq)]
enum Tile {
    Empty,
    ForwardMirror,
    BackwardMirror,
    HorizontalSplitter,
    VerticalSplitter,
}

struct Grid {
    bounds: IVec2,
    tiles: HashMap<IVec2, Tile>,
}

fn parse_grid(text: String) -> Grid {
    let mut tiles = HashMap::new();
    for (y, row) in text.lines().enumerate() {
        for (x, c) in row.chars().enumerate() {
            tiles.insert(
                IVec2::new(x as i32, y as i32),
                match c {
                    '.' => Tile::Empty,
                    '/' => Tile::ForwardMirror,
                    '\\' => Tile::BackwardMirror,
                    '-' => Tile::HorizontalSplitter,
                    '|' => Tile::VerticalSplitter,
                    _ => panic!("Failed to parse"),
                },
            );
        }
    }
    Grid {
        bounds: IVec2::new(
            text.lines().next().unwrap().len() as i32,
            text.lines().count() as i32,
        ),
        tiles,
    }
}

fn count_energized(grid: &Grid, start: (IVec2, Direction)) -> usize {
    // Store where the beam has been and what direction it was going when it entered
    let mut beam_path: HashMap<IVec2, HashSet<Direction>> = HashMap::new();
    let mut beam_heads = Vec::new();
    beam_heads.push(start);
    while let Some((loc, dir)) = beam_heads.pop() {
        if beam_path.contains_key(&loc) && beam_path.get(&loc).unwrap().contains(&dir) {
            continue;
        }
        match grid.tiles.get(&loc) {
            Some(Tile::Empty) => beam_heads.push(match dir {
                Direction::North => (loc - IVec2::new(0, 1), Direction::North),
                Direction::South => (loc + IVec2::new(0, 1), Direction::South),
                Direction::East => (loc + IVec2::new(1, 0), Direction::East),
                Direction::West => (loc - IVec2::new(1, 0), Direction::West),
            }),
            Some(Tile::ForwardMirror) => beam_heads.push(match dir {
                Direction::North => (loc + IVec2::new(1, 0), Direction::East),
                Direction::South => (loc - IVec2::new(1, 0), Direction::West),
                Direction::East => (loc - IVec2::new(0, 1), Direction::North),
                Direction::West => (loc + IVec2::new(0, 1), Direction::South),
            }),
            Some(Tile::BackwardMirror) => beam_heads.push(match dir {
                Direction::North => (loc - IVec2::new(1, 0), Direction::West),
                Direction::South => (loc + IVec2::new(1, 0), Direction::East),
                Direction::East => (loc + IVec2::new(0, 1), Direction::South),
                Direction::West => (loc - IVec2::new(0, 1), Direction::North),
            }),

            Some(Tile::HorizontalSplitter) => match dir {
                Direction::North | Direction::South => {
                    beam_heads.push((loc - IVec2::new(1, 0), Direction::West));
                    beam_heads.push((loc + IVec2::new(1, 0), Direction::East));
                }
                Direction::East => beam_heads.push((loc + IVec2::new(1, 0), Direction::East)),
                Direction::West => beam_heads.push((loc - IVec2::new(1, 0), Direction::West)),
            },
            Some(Tile::VerticalSplitter) => match dir {
                Direction::North => beam_heads.push((loc - IVec2::new(0, 1), Direction::North)),
                Direction::South => beam_heads.push((loc + IVec2::new(0, 1), Direction::South)),
                Direction::East | Direction::West => {
                    beam_heads.push((loc - IVec2::new(0, 1), Direction::North));
                    beam_heads.push((loc + IVec2::new(0, 1), Direction::South));
                }
            },

            None => {
                continue;
            }
        }
        beam_path.entry(loc).or_default().insert(dir);
    }
    beam_path.len()
}

pub fn part1(text: String) -> usize {
    let grid = parse_grid(text);
    count_energized(&grid, (IVec2::new(0, 0), Direction::East))
}

pub fn part2(text: String) -> usize {
    let grid = parse_grid(text);
    let mut max_energized = 0;
    for i in 0..grid.bounds.x {
        let energized = count_energized(&grid, (IVec2::new(i, 0), Direction::South));
        max_energized = energized.max(max_energized);
        let energized = count_energized(&grid, (IVec2::new(i, grid.bounds.y - 1), Direction::North));
        max_energized = energized.max(max_energized);
    }
    for i in 0..grid.bounds.y {
        let energized = count_energized(&grid, (IVec2::new(0, i), Direction::East));
        max_energized = energized.max(max_energized);
        let energized = count_energized(&grid, (IVec2::new(grid.bounds.x - 1, i), Direction::West));
        max_energized = energized.max(max_energized);
    }
    max_energized
}
