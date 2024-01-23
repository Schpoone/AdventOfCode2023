#![feature(int_roundings)]
use colored::Colorize;
use indicatif::ParallelProgressIterator;
use rayon::iter::IntoParallelIterator;
use rayon::prelude::*;
use std::collections::HashSet;

use glam::IVec2;
use itertools::{Itertools, MinMaxResult};

#[derive(Debug)]
struct Grid {
    start: IVec2,
    bounds: IVec2,
    rocks: HashSet<IVec2>,
}

impl Grid {
    /// Convert a location into a location within the bounds of the grid
    fn grid_bounded(&self, loc: &IVec2) -> IVec2 {
        IVec2::new(
            loc.x.rem_euclid(self.bounds.x),
            loc.y.rem_euclid(self.bounds.y),
        )
    }

    /// Get the closest coordinate between a grid covering coordinate and the start
    /// Assumes the start is in grid coordinate (0, 0)
    fn closest_grid_location(&self, loc: &IVec2) -> IVec2 {
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
        IVec2::new(x, y) + *loc * IVec2::splat(grid_size)
    }
}

fn parse_grid(text: String) -> Grid {
    let mut grid = Grid {
        start: IVec2::ZERO,
        bounds: IVec2::new(
            text.lines().next().unwrap().len() as i32,
            text.lines().count() as i32,
        ),
        rocks: HashSet::new(),
    };
    for (y, row) in text.lines().enumerate() {
        for (x, c) in row.chars().enumerate() {
            let loc = IVec2::new(x as i32, y as i32);
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

pub fn part1(text: String, num_steps: i32) -> usize {
    let grid = parse_grid(text);
    count_plots_with_bounded_grid(&grid, num_steps, &grid.start, 0, num_steps % 2)
}

fn count_plots_with_bounded_grid(
    grid: &Grid,
    num_steps: i32,
    start: &IVec2,
    start_parity: i32,
    final_parity: i32,
) -> usize {
    let mut final_plots = HashSet::new();
    let mut rejected_plots: HashSet<IVec2> = HashSet::new();
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
                    plot + IVec2::new(1, 0),
                    plot - IVec2::new(1, 0),
                    plot + IVec2::new(0, 1),
                    plot - IVec2::new(0, 1),
                ]
                .into_iter()
                .filter(|loc| {
                    loc.x >= 0
                        && loc.y >= 0
                        && loc.x < grid.bounds.x
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
    final_plots.len()
}

#[allow(dead_code)]
fn count_plots_with_unbounded_grid(
    grid: &Grid,
    num_steps: i32,
    start: &IVec2,
    start_parity: i32,
    final_parity: i32,
) -> usize {
    let mut final_plots = HashSet::new();
    let mut rejected_plots: HashSet<IVec2> = HashSet::new();
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
                    plot + IVec2::new(1, 0),
                    plot - IVec2::new(1, 0),
                    plot + IVec2::new(0, 1),
                    plot - IVec2::new(0, 1),
                ]
                .into_iter()
                .filter(|loc| {
                    let bounded_loc = grid.grid_bounded(loc);
                    !grid.rocks.contains(&bounded_loc)
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
    final_plots.len()
}

#[allow(dead_code)]
fn print_plots(grid: &Grid, plots: &HashSet<IVec2>) {
    let MinMaxResult::MinMax(x_min, x_max) = plots.iter().minmax_by_key(|plot| plot.x) else {return};
    let MinMaxResult::MinMax(y_min, y_max) = plots.iter().minmax_by_key(|plot| plot.y) else {return};
    for y in y_min.y..=y_max.y {
        for x in x_min.x..=x_max.x {
            let loc = IVec2::new(x, y);
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

fn distance(p1: &IVec2, p2: &IVec2) -> u32 {
    (p1.x.abs_diff(p2.x)) + (p1.y.abs_diff(p2.y))
}

pub fn part2(text: String, num_steps: i32) -> usize {
    let grid = parse_grid(text);
    // count_plots_with_unbounded_grid(&grid, num_steps, &grid.start, 0, num_steps % 2);
    let grid_size = grid.bounds.x; // This works because the grid is a square
    let grid_covering_radius = num_steps.div_ceil(grid_size);
    let even_grid_count =
        count_plots_with_bounded_grid(&grid, grid_size * 4, &grid.start, 0, 0);
    let odd_grid_count =
        count_plots_with_bounded_grid(&grid, grid_size * 4 + 1, &grid.start, 0, 1);
    // dbg!(even_grid_count);
    // dbg!(odd_grid_count);
    (-grid_covering_radius..=grid_covering_radius)
        .into_par_iter()
        // .into_iter()
        .progress_count(grid_covering_radius as u64 * 2 + 1)
        .map(|grid_y| {
            (-grid_covering_radius..=grid_covering_radius)
                .into_par_iter()
                // .into_iter()
                .map(|grid_x| {
                    let closest_grid_location =
                        grid.closest_grid_location(&IVec2::new(grid_x, grid_y));
                    let closest_grid_distance = distance(&closest_grid_location, &grid.start);
                    if closest_grid_distance > num_steps as u32 {
                        // Grid is fully outside the step radius
                        // println!("Outside radius");
                        0
                    } else if closest_grid_distance
                        < (num_steps as u32).saturating_sub(grid_size as u32 * 2)
                    {
                        // println!("Inside radius: {} < {}", closest_grid_distance, (num_steps - grid_size * 2));
                        // Grid is fully inside the step radius
                        if (grid_x + grid_y) % 2 == 0 {
                            even_grid_count
                        } else {
                            odd_grid_count
                        }
                    } else {
                        // println!("Close to radius");
                        // dbg!((grid_x, grid_y));
                        // dbg!(closest_grid_location);
                        // dbg!(count_plots_with_bounded_grid(
                        //     &grid,
                        //     num_steps - closest_grid_distance as i32,
                        //     &grid.grid_bounded(&closest_grid_location),
                        //     closest_grid_distance as i32 % 2,
                        //     num_steps % 2,
                        // ))
                        count_plots_with_bounded_grid(
                            &grid,
                            num_steps - closest_grid_distance as i32,
                            &grid.grid_bounded(&closest_grid_location),
                            closest_grid_distance as i32 % 2,
                            num_steps % 2,
                        )
                    }
                })
                .sum::<usize>()
        })
        .sum()
}
