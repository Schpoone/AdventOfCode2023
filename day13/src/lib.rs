use std::iter::zip;

use grid::Grid;

#[derive(Debug, PartialEq, Clone)]
enum Tile {
    Ash,
    Rock,
}

fn parse_grid(text: &str) -> Grid<Tile> {
    Grid::from_vec(
        text.replace('\n', "")
            .chars()
            .map(|c| match c {
                '.' => Tile::Ash,
                '#' => Tile::Rock,
                _ => panic!(),
            })
            .collect::<Vec<Tile>>(),
        text.lines().next().unwrap().len(),
    )
}

/// Finds each vertical and horizontal symmetry and returns the correct solution calculation for
/// this grid, i.e. number of columns to the left of each vertical line and 100 times the number of
/// rows above each horizontal line of reflection.
fn get_symmetries(grid: &Grid<Tile>, ignored_symmetry: usize) -> usize {
    let mut symmetries = 0;

    // Symmetries from vertical lines of reflection
    for x in 1..grid.cols() {
        if grid.iter_rows().all(|row| {
            let mut left = row.collect::<Vec<&Tile>>();
            let right = left.split_off(x);
            zip(left.iter().rev(), right.iter()).all(|(l, r)| *l == *r)
        }) && x != ignored_symmetry
        {
            symmetries += x;
        }
    }

    // Symmetries from horizontal lines of reflection
    for y in 1..grid.rows() {
        if grid.iter_cols().all(|col| {
            let mut top = col.collect::<Vec<&Tile>>();
            let bottom = top.split_off(y);
            zip(top.iter().rev(), bottom.iter()).all(|(t, b)| *t == *b)
        }) && 100 * y != ignored_symmetry
        {
            symmetries += 100 * y;
        }
    }

    symmetries
}

pub fn part1(text: String) -> usize {
    text.split("\n\n")
        .map(|grid_text| {
            let grid = parse_grid(grid_text);
            get_symmetries(&grid, 0)
        })
        .sum()
}

pub fn part2(text: String) -> usize {
    text.split("\n\n")
        .map(|grid_text| {
            let mut grid = parse_grid(grid_text);
            let orig_symmetries = get_symmetries(&grid, 0);
            for i in 0..grid.rows() {
                for j in 0..grid.cols() {
                    grid[(i, j)] = match grid[(i, j)] {
                        Tile::Ash => Tile::Rock,
                        Tile::Rock => Tile::Ash,
                    };
                    let new_symmetries = get_symmetries(&grid, orig_symmetries);
                    if new_symmetries == 0 {
                        grid[(i, j)] = match grid[(i, j)] {
                            Tile::Ash => Tile::Rock,
                            Tile::Rock => Tile::Ash,
                        };
                    } else {
                        return new_symmetries;
                    }
                }
            }
            panic!("Did not find a new symmetry!");
        })
        .sum()
}
