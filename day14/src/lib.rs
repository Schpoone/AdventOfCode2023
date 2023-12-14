use grid::Grid;
use indicatif::ProgressIterator;
use rayon::iter::{IntoParallelIterator, ParallelExtend, ParallelIterator};

#[derive(Debug, PartialEq, Eq, Clone)]
enum Tile {
    Round,
    Cube,
    Empty,
}

fn parse_grid(text: String) -> Grid<Tile> {
    Grid::from_vec(
        text.chars()
            .filter_map(|c| match c {
                'O' => Some(Tile::Round),
                '#' => Some(Tile::Cube),
                '.' => Some(Tile::Empty),
                _ => None,
            })
            .collect::<Vec<Tile>>(),
        text.lines().next().unwrap().len(),
    )
}

fn north(grid: Grid<Tile>) -> Grid<Tile> {
    let mut new_cols = Vec::new();
    new_cols.par_extend(
        grid.iter_cols()
            .collect::<Vec<_>>()
            .into_par_iter()
            .map(|col| {
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
                new_col
            }),
    );

    let mut new_grid = Grid::init(0, 0, Tile::Empty);
    for col in new_cols {
        new_grid.push_col(col);
    }
    new_grid
}

fn calc_load(grid: &Grid<Tile>) -> usize {
    grid.iter_rows().enumerate().fold(0, |acc, (i, row)| {
        acc + row.filter(|&t| *t == Tile::Round).count() * (grid.rows() - i)
    })
}

pub fn part1(text: String) -> usize {
    let grid = parse_grid(text);
    let grid = north(grid);
    calc_load(&grid)
}

fn west(grid: Grid<Tile>) -> Grid<Tile> {
    let mut new_rows = Vec::new();
    new_rows.par_extend(
        grid.iter_rows()
            .collect::<Vec<_>>()
            .into_par_iter()
            .map(|row| {
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
                new_row
            }),
    );

    let mut new_grid = Grid::init(0, 0, Tile::Empty);
    for row in new_rows {
        new_grid.push_row(row);
    }
    new_grid
}

fn south(grid: Grid<Tile>) -> Grid<Tile> {
    let mut new_cols = Vec::new();
    new_cols.par_extend(
        grid.iter_cols()
            .collect::<Vec<_>>()
            .into_par_iter()
            .map(|col| {
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
                new_col
            }),
    );

    let mut new_grid = Grid::init(0, 0, Tile::Empty);
    for col in new_cols {
        new_grid.push_col(col);
    }
    new_grid
}

fn east(grid: Grid<Tile>) -> Grid<Tile> {
    let mut new_rows = Vec::new();
    new_rows.par_extend(
        grid.iter_rows()
            .collect::<Vec<_>>()
            .into_par_iter()
            .map(|row| {
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
                new_row
            }),
    );

    let mut new_grid = Grid::init(0, 0, Tile::Empty);
    for row in new_rows {
        new_grid.push_row(row);
    }
    new_grid
}

pub fn part2(text: String) -> usize {
    let mut grid = parse_grid(text);
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(32)
        .build()
        .unwrap();
    pool.install(|| {
        for _ in (0..100).progress() {
            let old_grid = grid.clone();
            grid = north(grid);
            grid = west(grid);
            grid = south(grid);
            grid = east(grid);
            // dbg!(&grid);
            if grid == old_grid {
                break;
            }
            // if i % 10000000 == 0 {
            //     dbg!(old_grid);
            //     dbg!(&grid);
            // }
        }
        calc_load(&grid)
    })
}
