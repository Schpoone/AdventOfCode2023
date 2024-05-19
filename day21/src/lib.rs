use colored::Colorize;
use std::collections::HashSet;
use glam::UVec2;
use itertools::{Itertools, MinMaxResult};

#[derive(Debug)]
struct Grid {
    start: UVec2,
    bounds: UVec2,
    rocks: HashSet<UVec2>,
}

impl Grid {
    /// Convert a location into a location within the bounds of the grid
    fn grid_bounded(&self, loc: &UVec2) -> UVec2 {
        UVec2::new(
            loc.x.rem_euclid(self.bounds.x),
            loc.y.rem_euclid(self.bounds.y),
        )
    }

    /// Get the closest coordinate between a grid covering coordinate and the start
    /// Assumes the start is in grid coordinate (0, 0)
    fn closest_grid_location(&self, loc: &UVec2) -> UVec2 {
        let grid_size = self.bounds.x;
        let x = if loc.x < 0 {
            grid_size - 1
        } else if loc.x > 0 {
            0
        } else {
            self.start.x
        };
        let y = if loc.y < 0 {
            grid_size - 1
        } else if loc.y > 0 {
            0
        } else {
            self.start.y
        };
        UVec2::new(x, y) + *loc * UVec2::splat(grid_size)
    }
}

fn parse_grid(text: String) -> Grid {
    let mut grid = Grid {
        start: UVec2::ZERO,
        bounds: UVec2::new(
            text.lines().next().unwrap().len() as u32,
            text.lines().count() as u32,
        ),
        rocks: HashSet::new(),
    };
    for (y, row) in text.lines().enumerate() {
        for (x, c) in row.chars().enumerate() {
            let loc = UVec2::new(x as u32, y as u32);
            match c {
                '.' => {}
                '#' => {
                    grid.rocks.insert(loc);
                }
                'S' => {
                    grid.start = loc;
                }
                _ => panic!("Parsed bad character"),
            }
        }
    }
    grid
}

pub fn part1(text: String, num_steps: u32) -> u32 {
    let grid = parse_grid(text);
    count_plots_with_bounded_grid(&grid, num_steps, &grid.start, 0, num_steps % 2)
}

fn count_plots_with_bounded_grid(
    grid: &Grid,
    num_steps: u32,
    start: &UVec2,
    start_parity: u32,
    final_parity: u32,
) -> u32 {
    let mut final_plots = HashSet::new();
    let mut rejected_plots: HashSet<UVec2> = HashSet::new();
    let mut current_plots = HashSet::new();
    current_plots.insert(*start);
    if start_parity == final_parity {
        final_plots.insert(*start);
    }
    for i in 0..num_steps {
        let mut next_plots = HashSet::new();
        for plot in current_plots {
            next_plots.extend(
                [
                    plot + UVec2::new(1, 0),
                    plot - UVec2::new(1, 0),
                    plot + UVec2::new(0, 1),
                    plot - UVec2::new(0, 1),
                ]
                .into_iter()
                .filter(|loc| {
                        loc.x < grid.bounds.x
                        && loc.y < grid.bounds.y
                        && !grid.rocks.contains(loc)
                        && !final_plots.contains(loc)
                        && !rejected_plots.contains(loc)
                }),
            );
        }
        if (i + 1 + start_parity) % 2 == final_parity {
            final_plots.extend(next_plots.iter());
        } else {
            rejected_plots.extend(next_plots.iter());
        }
        current_plots = next_plots;
    }
    // print_plots(grid, &final_plots);
    final_plots.len() as u32
}

#[allow(dead_code)]
fn print_plots(grid: &Grid, plots: &HashSet<UVec2>) {
    let MinMaxResult::MinMax(x_min, x_max) = plots.iter().minmax_by_key(|plot| plot.x) else {
        return;
    };
    let MinMaxResult::MinMax(y_min, y_max) = plots.iter().minmax_by_key(|plot| plot.y) else {
        return;
    };
    for y in y_min.y..=y_max.y {
        for x in x_min.x..=x_max.x {
            let loc = UVec2::new(x, y);
            let bounded_loc = grid.grid_bounded(&loc);
            let tile_char = match grid.rocks.contains(&bounded_loc) {
                true => "#".clear(),
                false => ".".clear(),
            };
            let tile_char = match bounded_loc.x == 0
                || bounded_loc.y == 0
                || bounded_loc.x == grid.bounds.x - 1
                || bounded_loc.y == grid.bounds.y - 1
            {
                true => tile_char.blink(),
                false => tile_char,
            };
            if plots.contains(&loc) {
                print!("{}", tile_char.on_green());
            } else {
                print!("{}", tile_char);
            }
        }
        println!();
    }
    println!();
}

fn distance(p1: &UVec2, p2: &UVec2) -> u32 {
    (p1.x.abs_diff(p2.x)) + (p1.y.abs_diff(p2.y))
}

pub fn part2(text: String, num_steps: u32) -> u64 {
    let grid = parse_grid(text);
    assert!(grid.bounds.x == grid.bounds.y);
    let grid_size = grid.bounds.x;
    let half_grid_size = grid_size / 2;
    assert!((num_steps - half_grid_size) % grid_size == 0);
    let whole_grid_multiplier = (num_steps - half_grid_size) / grid_size;
    let whole_grid_count = count_plots_with_bounded_grid(&grid, grid_size, &grid.start, 0, grid_size % 2) as u64;
    let center_diamond_count = count_plots_with_bounded_grid(&grid, half_grid_size, &grid.start, 0, half_grid_size % 2) as u64;

    // These are the diamonds formed by the 4 corners that are interspersed among
    // the center diamonds to create a quilt like pattern
    let corner_diamond_count = whole_grid_count - center_diamond_count;

    let total_num_diamonds = (whole_grid_multiplier * 2 + 1).pow(2) as u64;
    let num_corner_diamonds = total_num_diamonds / 2;
    let num_center_diamonds = total_num_diamonds - num_corner_diamonds;
    assert!(num_center_diamonds + num_corner_diamonds == total_num_diamonds);
    num_center_diamonds * center_diamond_count + num_corner_diamonds * corner_diamond_count
}
