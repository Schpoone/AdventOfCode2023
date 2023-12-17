use pathfinding::prelude::dijkstra;
use std::collections::HashMap;

use glam::IVec2;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

struct Grid {
    bounds: IVec2,
    blocks: HashMap<IVec2, u32>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct State {
    loc: IVec2,
    dir: Direction,
    consecutive_blocks: i32,
}

fn parse_grid(text: String) -> Grid {
    let mut blocks = HashMap::new();
    for (y, row) in text.lines().enumerate() {
        for (x, c) in row.chars().enumerate() {
            blocks.insert(IVec2::new(x as i32, y as i32), c.to_digit(10).unwrap());
        }
    }
    Grid {
        bounds: IVec2::new(
            text.lines().next().unwrap().len() as i32,
            text.lines().count() as i32,
        ),
        blocks,
    }
}

pub fn part1(text: String) -> u32 {
    let grid = parse_grid(text);
    let (_path, total_heat_loss) = dijkstra(
        &State {
            loc: IVec2::new(0, 0),
            dir: Direction::East,
            consecutive_blocks: -1,
        },
        |s| {
            let mut successors = Vec::new();
            match s.dir {
                Direction::North => {
                    let next = State {
                        loc: s.loc - IVec2::new(1, 0),
                        dir: Direction::West,
                        consecutive_blocks: 0,
                    };
                    if let Some(heat_loss) = grid.blocks.get(&next.loc) {
                        successors.push((next, *heat_loss));
                    }
                    let next = State {
                        loc: s.loc + IVec2::new(1, 0),
                        dir: Direction::East,
                        consecutive_blocks: 0,
                    };
                    if let Some(heat_loss) = grid.blocks.get(&next.loc) {
                        successors.push((next, *heat_loss));
                    }
                    if s.consecutive_blocks < 2 {
                        let next = State {
                            loc: s.loc - IVec2::new(0, 1),
                            dir: Direction::North,
                            consecutive_blocks: s.consecutive_blocks + 1,
                        };
                        if let Some(heat_loss) = grid.blocks.get(&next.loc) {
                            successors.push((next, *heat_loss));
                        }
                    }
                }
                Direction::South => {
                    let next = State {
                        loc: s.loc - IVec2::new(1, 0),
                        dir: Direction::West,
                        consecutive_blocks: 0,
                    };
                    if let Some(heat_loss) = grid.blocks.get(&next.loc) {
                        successors.push((next, *heat_loss));
                    }
                    let next = State {
                        loc: s.loc + IVec2::new(1, 0),
                        dir: Direction::East,
                        consecutive_blocks: 0,
                    };
                    if let Some(heat_loss) = grid.blocks.get(&next.loc) {
                        successors.push((next, *heat_loss));
                    }
                    if s.consecutive_blocks < 2 {
                        let next = State {
                            loc: s.loc + IVec2::new(0, 1),
                            dir: Direction::South,
                            consecutive_blocks: s.consecutive_blocks + 1,
                        };
                        if let Some(heat_loss) = grid.blocks.get(&next.loc) {
                            successors.push((next, *heat_loss));
                        }
                    }
                }
                Direction::East => {
                    let next = State {
                        loc: s.loc - IVec2::new(0, 1),
                        dir: Direction::North,
                        consecutive_blocks: 0,
                    };
                    if let Some(heat_loss) = grid.blocks.get(&next.loc) {
                        successors.push((next, *heat_loss));
                    }
                    let next = State {
                        loc: s.loc + IVec2::new(0, 1),
                        dir: Direction::South,
                        consecutive_blocks: 0,
                    };
                    if let Some(heat_loss) = grid.blocks.get(&next.loc) {
                        successors.push((next, *heat_loss));
                    }
                    if s.consecutive_blocks < 2 {
                        let next = State {
                            loc: s.loc + IVec2::new(1, 0),
                            dir: Direction::East,
                            consecutive_blocks: s.consecutive_blocks + 1,
                        };
                        if let Some(heat_loss) = grid.blocks.get(&next.loc) {
                            successors.push((next, *heat_loss));
                        }
                    }
                }
                Direction::West => {
                    let next = State {
                        loc: s.loc - IVec2::new(0, 1),
                        dir: Direction::North,
                        consecutive_blocks: 0,
                    };
                    if let Some(heat_loss) = grid.blocks.get(&next.loc) {
                        successors.push((next, *heat_loss));
                    }
                    let next = State {
                        loc: s.loc + IVec2::new(0, 1),
                        dir: Direction::South,
                        consecutive_blocks: 0,
                    };
                    if let Some(heat_loss) = grid.blocks.get(&next.loc) {
                        successors.push((next, *heat_loss));
                    }
                    if s.consecutive_blocks < 2 {
                        let next = State {
                            loc: s.loc - IVec2::new(1, 0),
                            dir: Direction::West,
                            consecutive_blocks: s.consecutive_blocks + 1,
                        };
                        if let Some(heat_loss) = grid.blocks.get(&next.loc) {
                            successors.push((next, *heat_loss));
                        }
                    }
                }
            }
            successors
        },
        |s| s.loc == grid.bounds - IVec2::new(1, 1),
    )
    .unwrap();
    total_heat_loss
}

pub fn part2(text: String) -> u32 {
    let grid = parse_grid(text);
    let (_path, total_heat_loss) = dijkstra(
        &State {
            loc: IVec2::new(0, 0),
            dir: Direction::East,
            consecutive_blocks: -1,
        },
        |s| {
            let mut successors = Vec::new();
            match s.dir {
                Direction::North => {
                    if s.consecutive_blocks >= 3 {
                        let next = State {
                            loc: s.loc - IVec2::new(1, 0),
                            dir: Direction::West,
                            consecutive_blocks: 0,
                        };
                        if let Some(heat_loss) = grid.blocks.get(&next.loc) {
                            successors.push((next, *heat_loss));
                        }
                        let next = State {
                            loc: s.loc + IVec2::new(1, 0),
                            dir: Direction::East,
                            consecutive_blocks: 0,
                        };
                        if let Some(heat_loss) = grid.blocks.get(&next.loc) {
                            successors.push((next, *heat_loss));
                        }
                    }
                    if s.consecutive_blocks < 9 {
                        let next = State {
                            loc: s.loc - IVec2::new(0, 1),
                            dir: Direction::North,
                            consecutive_blocks: s.consecutive_blocks + 1,
                        };
                        if let Some(heat_loss) = grid.blocks.get(&next.loc) {
                            successors.push((next, *heat_loss));
                        }
                    }
                }
                Direction::South => {
                    if s.consecutive_blocks >= 3 {
                        let next = State {
                            loc: s.loc - IVec2::new(1, 0),
                            dir: Direction::West,
                            consecutive_blocks: 0,
                        };
                        if let Some(heat_loss) = grid.blocks.get(&next.loc) {
                            successors.push((next, *heat_loss));
                        }
                        let next = State {
                            loc: s.loc + IVec2::new(1, 0),
                            dir: Direction::East,
                            consecutive_blocks: 0,
                        };
                        if let Some(heat_loss) = grid.blocks.get(&next.loc) {
                            successors.push((next, *heat_loss));
                        }
                    }
                    if s.consecutive_blocks < 9 {
                        let next = State {
                            loc: s.loc + IVec2::new(0, 1),
                            dir: Direction::South,
                            consecutive_blocks: s.consecutive_blocks + 1,
                        };
                        if let Some(heat_loss) = grid.blocks.get(&next.loc) {
                            successors.push((next, *heat_loss));
                        }
                    }
                }
                Direction::East => {
                    if s.consecutive_blocks >= 3 {
                        let next = State {
                            loc: s.loc - IVec2::new(0, 1),
                            dir: Direction::North,
                            consecutive_blocks: 0,
                        };
                        if let Some(heat_loss) = grid.blocks.get(&next.loc) {
                            successors.push((next, *heat_loss));
                        }
                        let next = State {
                            loc: s.loc + IVec2::new(0, 1),
                            dir: Direction::South,
                            consecutive_blocks: 0,
                        };
                        if let Some(heat_loss) = grid.blocks.get(&next.loc) {
                            successors.push((next, *heat_loss));
                        }
                    }
                    if s.consecutive_blocks < 9 {
                        let next = State {
                            loc: s.loc + IVec2::new(1, 0),
                            dir: Direction::East,
                            consecutive_blocks: s.consecutive_blocks + 1,
                        };
                        if let Some(heat_loss) = grid.blocks.get(&next.loc) {
                            successors.push((next, *heat_loss));
                        }
                    }
                }
                Direction::West => {
                    if s.consecutive_blocks >= 3 {
                        let next = State {
                            loc: s.loc - IVec2::new(0, 1),
                            dir: Direction::North,
                            consecutive_blocks: 0,
                        };
                        if let Some(heat_loss) = grid.blocks.get(&next.loc) {
                            successors.push((next, *heat_loss));
                        }
                        let next = State {
                            loc: s.loc + IVec2::new(0, 1),
                            dir: Direction::South,
                            consecutive_blocks: 0,
                        };
                        if let Some(heat_loss) = grid.blocks.get(&next.loc) {
                            successors.push((next, *heat_loss));
                        }
                    }
                    if s.consecutive_blocks < 9 {
                        let next = State {
                            loc: s.loc - IVec2::new(1, 0),
                            dir: Direction::West,
                            consecutive_blocks: s.consecutive_blocks + 1,
                        };
                        if let Some(heat_loss) = grid.blocks.get(&next.loc) {
                            successors.push((next, *heat_loss));
                        }
                    }
                }
            }
            successors
        },
        |s| s.loc == grid.bounds - IVec2::new(1, 1) && s.consecutive_blocks >= 3,
    )
    .unwrap();
    total_heat_loss
}
