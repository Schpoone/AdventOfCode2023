#![feature(hash_extract_if)]
use std::collections::HashSet;

use colored::Colorize;

use glam::{IVec2, UVec2};

/// Direction we are traversing through pipe
/// CW = Clockwise, meaning the outside tiles are on the left
/// CCW = Counter-Clockwise, meaning the outside tiles are on the right
enum Direction {
    CW,
    CCW,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Tile {
    NS(UVec2),
    EW(UVec2),
    NE(UVec2),
    NW(UVec2),
    SW(UVec2),
    SE(UVec2),
    Ground(UVec2),
    Start(UVec2),
}

impl Tile {
    /// Given the tile location that you came from, get the next tile location
    /// Assumes the pipes are connected properly
    fn get_next(&self, from: UVec2) -> UVec2 {
        match self {
            Tile::NS(loc) => {
                if from.y == loc.y + 1 {
                    UVec2::new(loc.x, loc.y - 1)
                } else {
                    UVec2::new(loc.x, loc.y + 1)
                }
            }
            Tile::EW(loc) => {
                if from.x == loc.x + 1 {
                    UVec2::new(loc.x - 1, loc.y)
                } else {
                    UVec2::new(loc.x + 1, loc.y)
                }
            }
            Tile::NE(loc) => {
                if from.x == loc.x + 1 {
                    UVec2::new(loc.x, loc.y - 1)
                } else {
                    UVec2::new(loc.x + 1, loc.y)
                }
            }
            Tile::NW(loc) => {
                if from.x == loc.x - 1 {
                    UVec2::new(loc.x, loc.y - 1)
                } else {
                    UVec2::new(loc.x - 1, loc.y)
                }
            }
            Tile::SW(loc) => {
                if from.x == loc.x - 1 {
                    UVec2::new(loc.x, loc.y + 1)
                } else {
                    UVec2::new(loc.x - 1, loc.y)
                }
            }
            Tile::SE(loc) => {
                if from.x == loc.x + 1 {
                    UVec2::new(loc.x, loc.y + 1)
                } else {
                    UVec2::new(loc.x + 1, loc.y)
                }
            }
            Tile::Ground(_) => panic!("Next pipe segment is the ground?!"),
            Tile::Start(_) => panic!("Can't tell where the tile after start is"),
        }
    }

    /// Get all tiles considered to the left/right of the current pipes
    /// Next tile location needs to be given to determine direction
    /// Might be a location outside grid. If so, it's outside anyways
    fn get_side(&self, to: UVec2, left: bool) -> Vec<IVec2> {
        let to = to.as_ivec2();
        match self {
            Tile::NS(loc) => {
                let loc = loc.as_ivec2();
                if (to.y == loc.y + 1) == left {
                    vec![IVec2::new(loc.x + 1, loc.y)]
                } else {
                    vec![IVec2::new(loc.x - 1, loc.y)]
                }
            }
            Tile::EW(loc) => {
                let loc = loc.as_ivec2();
                if (to.x == loc.x + 1) == left {
                    vec![IVec2::new(loc.x, loc.y - 1)]
                } else {
                    vec![IVec2::new(loc.x, loc.y + 1)]
                }
            }
            Tile::NE(loc) => {
                let loc = loc.as_ivec2();
                if (to.x == loc.x + 1) == left {
                    vec![IVec2::new(loc.x + 1, loc.y - 1)]
                } else {
                    vec![
                        IVec2::new(loc.x - 1, loc.y),
                        IVec2::new(loc.x - 1, loc.y + 1),
                        IVec2::new(loc.x, loc.y + 1),
                    ]
                }
            }
            Tile::NW(loc) => {
                let loc = loc.as_ivec2();
                if (to.x == loc.x - 1) == left {
                    vec![
                        IVec2::new(loc.x + 1, loc.y),
                        IVec2::new(loc.x + 1, loc.y + 1),
                        IVec2::new(loc.x, loc.y + 1),
                    ]
                } else {
                    vec![IVec2::new(loc.x - 1, loc.y - 1)]
                }
            }
            Tile::SW(loc) => {
                let loc = loc.as_ivec2();
                if (to.x == loc.x - 1) == left {
                    vec![IVec2::new(loc.x - 1, loc.y + 1)]
                } else {
                    vec![
                        IVec2::new(loc.x + 1, loc.y),
                        IVec2::new(loc.x + 1, loc.y - 1),
                        IVec2::new(loc.x, loc.y - 1),
                    ]
                }
            }
            Tile::SE(loc) => {
                let loc = loc.as_ivec2();
                if (to.x == loc.x + 1) == left {
                    vec![
                        IVec2::new(loc.x - 1, loc.y),
                        IVec2::new(loc.x - 1, loc.y - 1),
                        IVec2::new(loc.x, loc.y - 1),
                    ]
                } else {
                    vec![IVec2::new(loc.x + 1, loc.y + 1)]
                }
            }
            Tile::Ground(_) => panic!("Not a pipe segment"),
            Tile::Start(_) => panic!("Can't tell what the side tiles are"),
        }
    }

    fn get_adj(&self, grid: &Grid) -> Vec<Tile> {
        [
            IVec2::new(1, 0),
            IVec2::new(0, 1),
            IVec2::new(-1, 0),
            IVec2::new(0, -1),
        ]
        .iter()
        .map(|offset| {
            (self.get_loc().as_ivec2() + *offset).clamp(
                IVec2::new(0, 0),
                IVec2::new(grid.tiles[0].len() as i32 - 1, grid.tiles.len() as i32 - 1),
            )
        })
        .map(|loc| grid.tiles[loc.y as usize][loc.x as usize])
        .collect()
    }

    fn get_loc(&self) -> UVec2 {
        match self {
            Tile::NS(l) => *l,
            Tile::EW(l) => *l,
            Tile::NE(l) => *l,
            Tile::NW(l) => *l,
            Tile::SW(l) => *l,
            Tile::SE(l) => *l,
            Tile::Ground(l) => *l,
            Tile::Start(l) => *l,
        }
    }

    fn is_connected_to_start(&self, start: UVec2) -> bool {
        match self {
            Tile::NS(loc) => start.y == loc.y + 1 || start.y == loc.y - 1,
            Tile::EW(loc) => start.x == loc.x + 1 || start.x == loc.x - 1,
            Tile::NE(loc) => start.x == loc.x + 1 || start.y == loc.y - 1,
            Tile::NW(loc) => start.x == loc.x - 1 || start.y == loc.y - 1,
            Tile::SW(loc) => start.x == loc.x - 1 || start.y == loc.y + 1,
            Tile::SE(loc) => start.x == loc.x + 1 || start.y == loc.y + 1,
            Tile::Ground(_) => false,
            Tile::Start(_) => false,
        }
    }

    fn fmt(&self) -> &str {
        match self {
            Tile::NS(_) => "|",
            Tile::EW(_) => "-",
            Tile::NE(_) => "L",
            Tile::NW(_) => "J",
            Tile::SW(_) => "7",
            Tile::SE(_) => "F",
            Tile::Ground(_) => ".",
            Tile::Start(_) => "S",
        }
    }
}

struct Grid {
    start: Tile,
    tiles: Vec<Vec<Tile>>,
}

impl Grid {
    fn out_of_bounds(&self, loc: IVec2) -> bool {
        loc.x < 0
            || loc.y < 0
            || loc.x >= self.tiles[0].len() as i32
            || loc.y >= self.tiles.len() as i32
    }
}

fn parse_grid(text: String) -> Grid {
    let mut grid = Grid {
        start: Tile::Start(UVec2::new(0, 0)),
        tiles: Vec::new(),
    };
    for (i, line) in text.lines().enumerate() {
        let mut row = Vec::new();
        for (j, c) in line.chars().enumerate() {
            row.push(match c {
                '|' => Tile::NS(UVec2::new(j as u32, i as u32)),
                '-' => Tile::EW(UVec2::new(j as u32, i as u32)),
                'L' => Tile::NE(UVec2::new(j as u32, i as u32)),
                'J' => Tile::NW(UVec2::new(j as u32, i as u32)),
                '7' => Tile::SW(UVec2::new(j as u32, i as u32)),
                'F' => Tile::SE(UVec2::new(j as u32, i as u32)),
                '.' => Tile::Ground(UVec2::new(j as u32, i as u32)),
                'S' => {
                    grid.start = Tile::Start(UVec2::new(j as u32, i as u32));
                    Tile::Start(UVec2::new(j as u32, i as u32))
                }
                _ => panic!("Parsed bad character"),
            });
        }
        grid.tiles.push(row);
    }
    grid
}

pub fn part1(text: String) -> u32 {
    let grid = parse_grid(text);

    // From the starting tile, follow the loop and divide the loop length by 2
    let mut prev = grid.start;
    let adj_tiles = prev
        .get_adj(&grid)
        .iter()
        .filter_map(|tile| {
            if tile.is_connected_to_start(prev.get_loc()) {
                Some(*tile)
            } else {
                None
            }
        })
        .collect::<Vec<Tile>>();
    debug_assert!(adj_tiles.len() == 2);

    let mut cur = adj_tiles[0];
    let mut loop_length = 1;
    while cur != grid.start {
        let next_loc = cur.get_next(prev.get_loc());
        loop_length += 1;
        prev = cur;
        cur = grid.tiles[next_loc.y as usize][next_loc.x as usize];
    }

    loop_length / 2
}

pub fn part2(text: String) -> usize {
    let grid = parse_grid(text);
    let mut remaining_tiles: HashSet<Tile> = HashSet::new();
    for row in grid.tiles.iter() {
        for tile in row.iter() {
            remaining_tiles.insert(*tile);
        }
    }

    // Find the loop as the first pass and mark tiles as part of the loop
    let mut in_loop: HashSet<Tile> = HashSet::new();
    in_loop.insert(grid.start);
    remaining_tiles.remove(&grid.start);
    let mut prev = grid.start;
    let adj_tiles = prev
        .get_adj(&grid)
        .iter()
        .filter_map(|tile| {
            if tile.is_connected_to_start(prev.get_loc()) {
                Some(*tile)
            } else {
                None
            }
        })
        .collect::<Vec<Tile>>();
    debug_assert!(adj_tiles.len() == 2);
    let mut cur = adj_tiles[0];
    while cur != grid.start {
        in_loop.insert(cur);
        remaining_tiles.remove(&cur);
        let next_loc = cur.get_next(prev.get_loc());
        prev = cur;
        cur = grid.tiles[next_loc.y as usize][next_loc.x as usize];
    }

    // Debug print grid
    println!("After marking looped pipe");
    for row in grid.tiles.iter() {
        for tile in row.iter() {
            if in_loop.contains(tile) {
                print!("{}", tile.fmt().yellow());
            } else {
                print!("{}", tile.fmt());
            }
        }
        println!();
    }

    // Mark the borders as outside
    let mut outside: HashSet<Tile> = HashSet::new();
    for tile in remaining_tiles.extract_if(|tile| {
        let loc = tile.get_loc();
        loc.x == 0
            || loc.y == 0
            || loc.x == (grid.tiles[0].len() as u32 - 1)
            || loc.y == (grid.tiles.len() as u32 - 1)
    }) {
        outside.insert(tile);
    }

    // Debug print grid
    println!("After marking borders");
    for row in grid.tiles.iter() {
        for tile in row.iter() {
            if in_loop.contains(tile) {
                print!("{}", tile.fmt().yellow());
            } else if outside.contains(tile) {
                print!("{}", tile.fmt().red());
            } else {
                print!("{}", tile.fmt());
            }
        }
        println!();
    }

    let mut prev_outside = outside.clone();
    loop {
        // Propagate adjacencies
        let mut next_outside = outside.clone();
        loop {
            for tile in remaining_tiles
                .extract_if(|tile| tile.get_adj(&grid).iter().any(|adj| outside.contains(adj)))
            {
                next_outside.insert(tile);
            }
            if outside.len() == next_outside.len() {
                break;
            }
            outside = next_outside.clone();
        }

        // Debug print grid
        println!("After propagating");
        for row in grid.tiles.iter() {
            for tile in row.iter() {
                if in_loop.contains(tile) {
                    print!("{}", tile.fmt().yellow());
                } else if outside.contains(tile) {
                    print!("{}", tile.fmt().red());
                } else {
                    print!("{}", tile.fmt());
                }
            }
            println!();
        }

        // Go through the loop in one direction
        // all tiles on the same side of the loop should be grouped
        // i.e. if the left side contains an outside tile, every left side tile is an outside tile
        let adj_tiles = prev
            .get_adj(&grid)
            .iter()
            .filter_map(|tile| {
                if tile.is_connected_to_start(prev.get_loc()) {
                    Some(*tile)
                } else {
                    None
                }
            })
            .collect::<Vec<Tile>>();
        let mut cur = adj_tiles[0];
        let mut direction = Option::None;
        while cur != grid.start {
            let next_loc = cur.get_next(prev.get_loc());
            if cur.get_side(next_loc, true).into_iter().any(|side_loc| {
                if grid.out_of_bounds(side_loc) {
                    return true;
                }
                let tile = grid.tiles[side_loc.y as usize][side_loc.x as usize];
                outside.contains(&tile)
            }) {
                direction = Some(Direction::CW);
                break;
            }
            if cur.get_side(next_loc, false).into_iter().any(|side_loc| {
                if grid.out_of_bounds(side_loc) {
                    return true;
                }
                let tile = grid.tiles[side_loc.y as usize][side_loc.x as usize];
                outside.contains(&tile)
            }) {
                direction = Some(Direction::CCW);
                break;
            }
            prev = cur;
            cur = grid.tiles[next_loc.y as usize][next_loc.x as usize];
        }
        if direction.is_some() {
            while cur != grid.start {
                let next_loc = cur.get_next(prev.get_loc());
                for side_loc in cur.get_side(next_loc, true).into_iter() {
                    if grid.out_of_bounds(side_loc) {
                        continue;
                    }
                    let side_tile = grid.tiles[side_loc.y as usize][side_loc.x as usize];
                    if !remaining_tiles.contains(&side_tile) {
                        continue;
                    }
                    if let Some(Direction::CW) = direction {
                        outside.insert(side_tile);
                        remaining_tiles.remove(&side_tile);
                    }
                }
                for side_loc in cur.get_side(next_loc, false).into_iter() {
                    if grid.out_of_bounds(side_loc) {
                        continue;
                    }
                    let side_tile = grid.tiles[side_loc.y as usize][side_loc.x as usize];
                    if !remaining_tiles.contains(&side_tile) {
                        continue;
                    }
                    if let Some(Direction::CCW) = direction {
                        outside.insert(side_tile);
                        remaining_tiles.remove(&side_tile);
                    }
                }
                prev = cur;
                cur = grid.tiles[next_loc.y as usize][next_loc.x as usize];
            }
        }

        // Debug print grid
        println!("After marking tiles adjacent to pipe");
        for row in grid.tiles.iter() {
            for tile in row.iter() {
                if in_loop.contains(tile) {
                    print!("{}", tile.fmt().yellow());
                } else if outside.contains(tile) {
                    print!("{}", tile.fmt().red());
                } else {
                    print!("{}", tile.fmt());
                }
            }
            println!();
        }

        if prev_outside.len() == outside.len() {
            break;
        }
        prev_outside = outside.clone();
    }

    // Debug print grid
    for row in grid.tiles.iter() {
        for tile in row.iter() {
            if in_loop.contains(tile) {
                print!("{}", tile.fmt().yellow());
            } else if outside.contains(tile) {
                print!("{}", tile.fmt().red());
            } else {
                print!("{}", tile.fmt().green());
            }
        }
        println!();
    }

    remaining_tiles.len()
}
