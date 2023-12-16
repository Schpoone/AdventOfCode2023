use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};

use grid::Grid;
use indicatif::ProgressIterator;

#[derive(PartialEq, Eq, Clone)]
struct HashableGrid<T: Eq> {
    grid: Grid<T>,
}

impl<T: Hash + Clone + Eq> Hash for HashableGrid<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.grid.clone().into_vec().hash(state);
    }
}

impl<T: Eq + Debug> Debug for HashableGrid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.grid.iter_rows() {
            for c in row {
                write!(f, "{:?}", c)?;
            }
            writeln!(f)?;
        }
        writeln!(f)
    }
}

#[derive(PartialEq, Eq, Clone, Hash)]
enum Tile {
    Round,
    Cube,
    Empty,
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Tile::Round => 'O',
            Tile::Cube => '#',
            Tile::Empty => '.',
        })
    }
}

fn parse_grid(text: String) -> HashableGrid<Tile> {
    HashableGrid {
        grid: Grid::from_vec(
            text.chars()
                .filter_map(|c| match c {
                    'O' => Some(Tile::Round),
                    '#' => Some(Tile::Cube),
                    '.' => Some(Tile::Empty),
                    _ => None,
                })
                .collect::<Vec<Tile>>(),
            text.lines().next().unwrap().len(),
        ),
    }
}

fn north(grid: HashableGrid<Tile>) -> HashableGrid<Tile> {
    let mut new_grid = Grid::init(0, 0, Tile::Empty);
    for col in grid.grid.iter_cols() {
        let mut new_col = Vec::new();
        let mut idx = 0;
        for (i, tile) in col.clone().enumerate() {
            match tile {
                Tile::Round => {
                    new_col.push(Tile::Round);
                    idx += 1;
                }
                Tile::Cube => {
                    while idx < i {
                        new_col.push(Tile::Empty);
                        idx += 1;
                    }
                    new_col.push(Tile::Cube);
                    idx += 1;
                }
                Tile::Empty => {}
            }
        }
        while new_col.len() < col.clone().count() {
            new_col.push(Tile::Empty);
        }
        new_grid.push_col(new_col);
    }
    HashableGrid { grid: new_grid }
}

fn calc_load(grid: &HashableGrid<Tile>) -> usize {
    grid.grid.iter_rows().enumerate().fold(0, |acc, (i, row)| {
        acc + row.filter(|&t| *t == Tile::Round).count() * (grid.grid.rows() - i)
    })
}

pub fn part1(text: String) -> usize {
    let grid = parse_grid(text);
    let grid = north(grid);
    calc_load(&grid)
}

fn west(grid: HashableGrid<Tile>) -> HashableGrid<Tile> {
    let mut new_grid = Grid::init(0, 0, Tile::Empty);
    for row in grid.grid.iter_rows() {
        let mut new_row = Vec::new();
        let mut idx = 0;
        for (i, tile) in row.clone().enumerate() {
            match tile {
                Tile::Round => {
                    new_row.push(Tile::Round);
                    idx += 1;
                }
                Tile::Cube => {
                    while idx < i {
                        new_row.push(Tile::Empty);
                        idx += 1;
                    }
                    new_row.push(Tile::Cube);
                    idx += 1;
                }
                Tile::Empty => {}
            }
        }
        while new_row.len() < row.clone().count() {
            new_row.push(Tile::Empty);
        }
        new_grid.push_row(new_row);
    }
    HashableGrid { grid: new_grid }
}

fn south(grid: HashableGrid<Tile>) -> HashableGrid<Tile> {
    let mut new_grid = Grid::init(0, 0, Tile::Empty);
    for col in grid.grid.iter_cols() {
        let mut new_col = Vec::new();
        let mut idx = 0;
        for (i, tile) in col.clone().rev().enumerate() {
            match tile {
                Tile::Round => {
                    new_col.insert(0, Tile::Round);
                    idx += 1;
                }
                Tile::Cube => {
                    while idx < i {
                        new_col.insert(0, Tile::Empty);
                        idx += 1;
                    }
                    new_col.insert(0, Tile::Cube);
                    idx += 1;
                }
                Tile::Empty => {}
            }
        }
        while new_col.len() < col.clone().count() {
            new_col.insert(0, Tile::Empty);
        }
        new_grid.push_col(new_col);
    }
    HashableGrid { grid: new_grid }
}

fn east(grid: HashableGrid<Tile>) -> HashableGrid<Tile> {
    let mut new_grid = Grid::init(0, 0, Tile::Empty);
    for row in grid.grid.iter_rows() {
        let mut new_row = Vec::new();
        let mut idx = 0;
        for (i, tile) in row.clone().rev().enumerate() {
            match tile {
                Tile::Round => {
                    new_row.insert(0, Tile::Round);
                    idx += 1;
                }
                Tile::Cube => {
                    while idx < i {
                        new_row.insert(0, Tile::Empty);
                        idx += 1;
                    }
                    new_row.insert(0, Tile::Cube);
                    idx += 1;
                }
                Tile::Empty => {}
            }
        }
        while new_row.len() < row.clone().count() {
            new_row.insert(0, Tile::Empty);
        }
        new_grid.push_row(new_row);
    }
    HashableGrid { grid: new_grid }
}

pub fn part2(text: String) -> usize {
    let mut grid = parse_grid(text);
    let mut state_cache: HashMap<HashableGrid<Tile>, i32> = HashMap::new();
    let num_cycles = 1000000000;
    for i in (0..num_cycles).progress() {
        grid = north(grid);
        grid = west(grid);
        grid = south(grid);
        grid = east(grid);
        if state_cache.contains_key(&grid) {
            let first_idx = state_cache.get(&grid).unwrap();
            let cycle_len = i - first_idx;
            grid = state_cache.iter().find(|(_,&v)| {
                v == first_idx + ((num_cycles - first_idx - 1) % cycle_len)
            }).expect("Didn't find the grid").0.clone();
            break;
        }
        state_cache.insert(grid.clone(), i);
    }
    calc_load(&grid)
}
