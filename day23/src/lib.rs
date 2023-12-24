use core::fmt;
use std::collections::{HashMap, HashSet};

use colored::Colorize;
use glam::IVec2;
use petgraph::{algo::all_simple_paths, Graph};

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Path,
    Forest,
    Slope(Direction),
}

struct Grid {
    bounds: IVec2,
    tiles: HashMap<IVec2, Tile>,
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f)?;
        for y in 0..self.bounds.y {
            for x in 0..self.bounds.x {
                let loc = IVec2::new(x, y);
                let tile = self.tiles.get(&loc).unwrap();
                let tile_char = match tile {
                    Tile::Path => ".",
                    Tile::Forest => "#",
                    Tile::Slope(Direction::North) => "^",
                    Tile::Slope(Direction::East) => ">",
                    Tile::Slope(Direction::South) => "v",
                    Tile::Slope(Direction::West) => "<",
                };
                write!(f, "{}", tile_char)?;
            }
            writeln!(f)?;
        }
        writeln!(f)
    }
}

impl Grid {
    #[allow(dead_code)]
    fn show_path(&self, path: &Path) {
        println!();
        for y in 0..self.bounds.y {
            for x in 0..self.bounds.x {
                let loc = IVec2::new(x, y);
                let tile = self.tiles.get(&loc).unwrap();
                let tile_char = match tile {
                    Tile::Path => ".",
                    Tile::Forest => "#",
                    Tile::Slope(Direction::North) => "^",
                    Tile::Slope(Direction::East) => ">",
                    Tile::Slope(Direction::South) => "v",
                    Tile::Slope(Direction::West) => "<",
                };
                if path.visited.contains(&loc) {
                    print!("{}", tile_char.green());
                } else {
                    print!("{}", tile_char);
                }
            }
            println!();
        }
        println!();
    }
    fn get_start(&self) -> IVec2 {
        for x in 0..self.bounds.x {
            let loc = IVec2::new(x, 0);
            if self.tiles.get(&loc) == Some(&Tile::Path) {
                return loc;
            }
        }
        panic!("No start tile");
    }
    fn get_end(&self) -> IVec2 {
        for x in 0..self.bounds.x {
            let loc = IVec2::new(x, self.bounds.y - 1);
            if self.tiles.get(&loc) == Some(&Tile::Path) {
                return loc;
            }
        }
        panic!("No end tile");
    }
}

#[derive(Debug, Clone)]
struct Path {
    visited: HashSet<IVec2>,
    current: IVec2,
}

fn parse_grid(text: String) -> Grid {
    let mut grid = Grid {
        bounds: IVec2::new(
            text.lines().next().unwrap().len() as i32,
            text.lines().count() as i32,
        ),
        tiles: HashMap::new(),
    };
    for (y, row) in text.lines().enumerate() {
        for (x, c) in row.chars().enumerate() {
            let loc = IVec2::new(x as i32, y as i32);
            grid.tiles.insert(
                loc,
                match c {
                    '.' => Tile::Path,
                    '#' => Tile::Forest,
                    '^' => Tile::Slope(Direction::North),
                    '>' => Tile::Slope(Direction::East),
                    'v' => Tile::Slope(Direction::South),
                    '<' => Tile::Slope(Direction::West),
                    _ => panic!("Parsed bad character"),
                },
            );
        }
    }
    grid
}

pub fn part1(text: String) -> usize {
    let grid = parse_grid(text);
    let start = grid.get_start();
    let end = grid.get_end();
    let mut current_paths = Vec::<Path>::new();
    let mut finished_paths = Vec::<Path>::new();
    current_paths.push(Path {
        visited: HashSet::new(),
        current: start,
    });
    while let Some(path) = current_paths.pop() {
        if path.current == end {
            finished_paths.push(path);
        } else {
            [
                path.current + IVec2::new(1, 0),
                path.current - IVec2::new(1, 0),
                path.current + IVec2::new(0, 1),
                path.current - IVec2::new(0, 1),
            ]
            .into_iter()
            .for_each(|adj_loc| {
                if let Some(tile) = grid.tiles.get(&adj_loc) {
                    let mut path = path.clone();
                    match tile {
                        Tile::Path => {
                            if !path.visited.contains(&adj_loc) {
                                path.visited.insert(adj_loc);
                                path.current = adj_loc;
                                current_paths.push(path);
                            }
                        }
                        Tile::Forest => {}
                        Tile::Slope(Direction::North) => {
                            let next_loc = adj_loc - IVec2::new(0, 1);
                            if !path.visited.contains(&adj_loc) && !path.visited.contains(&next_loc)
                            {
                                path.visited.insert(adj_loc);
                                path.visited.insert(next_loc);
                                path.current = next_loc;
                                current_paths.push(path);
                            }
                        }
                        Tile::Slope(Direction::South) => {
                            let next_loc = adj_loc + IVec2::new(0, 1);
                            if !path.visited.contains(&adj_loc) && !path.visited.contains(&next_loc)
                            {
                                path.visited.insert(adj_loc);
                                path.visited.insert(next_loc);
                                path.current = next_loc;
                                current_paths.push(path);
                            }
                        }
                        Tile::Slope(Direction::East) => {
                            let next_loc = adj_loc + IVec2::new(1, 0);
                            if !path.visited.contains(&adj_loc) && !path.visited.contains(&next_loc)
                            {
                                path.visited.insert(adj_loc);
                                path.visited.insert(next_loc);
                                path.current = next_loc;
                                current_paths.push(path);
                            }
                        }
                        Tile::Slope(Direction::West) => {
                            let next_loc = adj_loc - IVec2::new(1, 0);
                            if !path.visited.contains(&adj_loc) && !path.visited.contains(&next_loc)
                            {
                                path.visited.insert(adj_loc);
                                path.visited.insert(next_loc);
                                path.current = next_loc;
                                current_paths.push(path);
                            }
                        }
                    }
                }
            })
        }
    }
    finished_paths
        .into_iter()
        .map(|path| path.visited.len())
        .max()
        .unwrap()
}

pub fn part2(text: String) -> usize {
    let grid = parse_grid(text);
    let start = grid.get_start();
    let end = grid.get_end();

    // Construct a graph from the paths
    let mut graph = Graph::new();
    let mut node_id_map = HashMap::new();
    let mut visited = HashSet::new();
    let mut current_locs = Vec::new();
    node_id_map.insert(start, graph.add_node(start));
    current_locs.push(start);
    while let Some(loc) = current_locs.pop() {
        let node_id = *node_id_map.get(&loc).unwrap();
        [
            loc + IVec2::new(1, 0),
            loc - IVec2::new(1, 0),
            loc + IVec2::new(0, 1),
            loc - IVec2::new(0, 1),
        ]
        .into_iter()
        .for_each(|adj_loc| {
            if let Some(tile) = grid.tiles.get(&adj_loc) {
                match tile {
                    Tile::Path | Tile::Slope(_) => {
                        if !visited.contains(&adj_loc) {
                            node_id_map.insert(adj_loc, graph.add_node(adj_loc));
                            visited.insert(adj_loc);
                            current_locs.push(adj_loc);
                        }
                        graph.update_edge(node_id, *node_id_map.get(&adj_loc).unwrap(), 1);
                    }
                    Tile::Forest => {}
                }
            }
        })
    }
    all_simple_paths(
        &graph,
        *node_id_map.get(&start).unwrap(),
        *node_id_map.get(&end).unwrap(),
        1,
        None,
    )
    .max_by_key(|path: &Vec<_>| path.len()).unwrap().len() - 1
}
