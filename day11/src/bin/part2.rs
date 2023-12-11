use std::fs;
use day11::part2;

fn main() {
    let text = fs::read_to_string("data/input.txt").unwrap();
    println!("{}", part2(text));
}
