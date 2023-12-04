use std::cmp::max;

#[derive(Debug)]
struct Coord(usize, usize);

impl Coord {
    /// Chebyshev distance between two coordinates
    fn dist(&self, other: &Coord) -> usize {
        max(self.0.abs_diff(other.0), self.1.abs_diff(other.1))
    }

    /// When self is the Coord for the start of a string and
    /// length is the length of that string,
    /// find the minimum Chebyshev distance between that string and the other Coord
    fn min_dist(&self, other: &Coord, length: usize) -> usize {
        let mut min = usize::MAX;
        for n in 0..length {
            let c = Coord(self.0, self.1 + n);
            let d = c.dist(other);
            if d < min {
                min = d;
            }
        }
        min
    }
}

#[derive(Debug)]
struct Schematic {
    part_nums: Vec<(String, Coord)>,
    symbols: Vec<(char, Coord)>,
}

fn parse(text: String) -> Schematic {
    let mut schematic = Schematic {
        part_nums: Vec::new(),
        symbols: Vec::new(),
    };
    for (i, line) in text.lines().enumerate() {
        let mut in_num = false;
        let mut cur_num = String::new();
        for (j, c) in line.chars().enumerate() {
            match (in_num, c) {
                (true, c) if c.is_numeric() => cur_num.push(c),
                (true, c) => {
                    schematic
                        .part_nums
                        .push((cur_num.clone(), Coord(i, j - cur_num.len())));
                    in_num = false;
                    cur_num = String::new();
                    if c != '.' {
                        schematic.symbols.push((c, Coord(i, j)));
                    }
                }
                (false, c) if c.is_numeric() => {
                    in_num = true;
                    cur_num.push(c);
                }
                (false, c) => {
                    if c != '.' {
                        schematic.symbols.push((c, Coord(i, j)));
                    }
                }
            }
        }
        if in_num {
            schematic
                .part_nums
                .push((cur_num.clone(), Coord(i, line.len() - cur_num.len())))
        }
    }
    schematic
}

pub fn part1(text: String) -> u32 {
    let schematic = parse(text);
    schematic
        .part_nums
        .iter()
        .filter_map(|(num, start)| {
            for sym in schematic.symbols.iter() {
                if start.min_dist(&sym.1, num.len()) == 1 {
                    return Some(num.parse::<u32>().unwrap());
                }
            }
            None
        })
        .sum()
}

pub fn part2(text: String) -> u32 {
    let schematic = parse(text);
    schematic
        .symbols
        .iter()
        .filter_map(|(sym, coord)| {
            if sym != &'*' {
                return None;
            }
            let mut adj_nums = Vec::new();
            for num in schematic.part_nums.iter() {
                if num.1.min_dist(coord, num.0.len()) == 1 {
                    adj_nums.push(num.0.parse::<u32>().unwrap());
                }
            }
            if adj_nums.len() == 2 {
                Some(adj_nums[0] * adj_nums[1])
            } else {
                None
            }
        })
        .sum()
}
