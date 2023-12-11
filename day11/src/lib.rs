use core::fmt;
use std::collections::HashSet;

use glam::U64Vec2;

#[derive(Debug)]
struct Grid {
    bounds: U64Vec2,
    galaxies: HashSet<U64Vec2>,
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f)?;
        for y in 0..self.bounds.y {
            for x in 0..self.bounds.x {
                if self.galaxies.contains(&U64Vec2::new(x, y)) {
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

fn parse_grid(text: String) -> Grid {
    let mut galaxies = HashSet::new();
    for (i, row) in text.lines().enumerate() {
        for (j, c) in row.chars().enumerate() {
            if c == '#' {
                galaxies.insert(U64Vec2::new(j as u64, i as u64));
            }
        }
    }
    Grid {
        bounds: U64Vec2::new(
            text.lines().next().unwrap().len() as u64,
            text.lines().count() as u64,
        ),
        galaxies,
    }
}

fn expand(grid: Grid, expansion_ratio: u64) -> Grid {
    let mut empty_cols = Vec::new();
    for x in 0..grid.bounds.x {
        if grid.galaxies.iter().all(|loc| loc.x != x) {
            empty_cols.push(x);
        }
    }
    let mut empty_rows = Vec::new();
    for y in 0..grid.bounds.y {
        if grid.galaxies.iter().all(|loc| loc.y != y) {
            empty_rows.push(y);
        }
    }
    let mut expanded_galaxies = HashSet::new();
    for galaxy in grid.galaxies.iter() {
        expanded_galaxies.insert(U64Vec2::new(
            galaxy.x
                + empty_cols.iter().filter(|&&x| galaxy.x > x).count() as u64
                    * (expansion_ratio - 1),
            galaxy.y
                + empty_rows.iter().filter(|&&y| galaxy.y > y).count() as u64
                    * (expansion_ratio - 1),
        ));
    }
    Grid {
        bounds: U64Vec2::new(
            grid.bounds.x + empty_cols.len() as u64,
            grid.bounds.y + empty_rows.len() as u64,
        ),
        galaxies: expanded_galaxies,
    }
}

fn dist(g1: &U64Vec2, g2: &U64Vec2) -> u64 {
    g1.x.abs_diff(g2.x) + g1.y.abs_diff(g2.y)
}

pub fn part1(text: String) -> u64 {
    let grid = parse_grid(text);
    let grid = expand(grid, 2);
    let mut sum = 0;
    for galaxy1 in grid.galaxies.iter() {
        for galaxy2 in grid.galaxies.iter() {
            sum += dist(galaxy1, galaxy2);
        }
    }
    sum / 2
}

pub fn part2(text: String) -> u64 {
    let grid = parse_grid(text);
    let grid = expand(grid, 1000000);
    let mut sum = 0;
    for galaxy1 in grid.galaxies.iter() {
        for galaxy2 in grid.galaxies.iter() {
            sum += dist(galaxy1, galaxy2);
        }
    }
    sum / 2
}
