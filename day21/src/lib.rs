use colored::Colorize;
use indicatif::ProgressIterator;
use std::collections::HashSet;

use glam::IVec2;
use itertools::{Itertools, MinMaxResult};

#[derive(Debug)]
struct Grid {
    start: IVec2,
    bounds: IVec2,
    rocks: HashSet<IVec2>,
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

pub fn part1(text: String, num_steps: u32) -> usize {
    let grid = parse_grid(text);
    let mut current_plots = HashSet::new();
    current_plots.insert(grid.start);
    for _ in 0..num_steps {
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
                .filter(|v| v.x < grid.bounds.x && v.y < grid.bounds.y && !grid.rocks.contains(v)),
            );
        }
        current_plots = next_plots;
    }
    current_plots.len()
}

fn print_plots(grid: &Grid, plots: &HashSet<IVec2>) {
    let MinMaxResult::MinMax(x_min, x_max) = plots.iter().minmax_by_key(|plot| plot.x) else {return};
    let MinMaxResult::MinMax(y_min, y_max) = plots.iter().minmax_by_key(|plot| plot.y) else {return};
    for y in y_min.y..=y_max.y {
        for x in x_min.x..=x_max.x {
            let loc = IVec2::new(x, y);
            let mut bounded_loc = loc;
            while bounded_loc.x < 0 {
                bounded_loc.x += grid.bounds.x
            }
            while bounded_loc.y < 0 {
                bounded_loc.y += grid.bounds.y
            }
            let bounded_loc =
                IVec2::new(bounded_loc.x % grid.bounds.x, bounded_loc.y % grid.bounds.y);
            let tile = grid.rocks.get(&bounded_loc);
            let tile_char = match tile {
                Some(_) => "#",
                None => ".",
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

pub fn part2(text: String, num_steps: u32) -> usize {
    let grid = parse_grid(text);
    let mut final_plots = HashSet::new();
    let mut rejected_plots: HashSet<IVec2> = HashSet::new();
    let mut current_plots = HashSet::new();
    current_plots.insert(grid.start);
    if num_steps % 2 == 0 {
        final_plots.insert(grid.start);
    }
    for i in (0..num_steps).progress() {
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
                    let mut bounded_loc = *loc;
                    while bounded_loc.x < 0 {
                        bounded_loc.x += grid.bounds.x
                    }
                    while bounded_loc.y < 0 {
                        bounded_loc.y += grid.bounds.y
                    }
                    let bounded_loc =
                        IVec2::new(bounded_loc.x % grid.bounds.x, bounded_loc.y % grid.bounds.y);
                    !grid.rocks.contains(&bounded_loc)
                        && !final_plots.contains(loc)
                        && !rejected_plots.contains(loc)
                }),
            );
        }
        if (i + 1) % 2 == num_steps % 2 {
            final_plots.extend(next_plots.iter());
        } else {
            rejected_plots.extend(next_plots.iter());
        }
        current_plots = next_plots;
        print_plots(&grid, &final_plots);
    }
    final_plots.len()
}
