use std::fs;
use day21::part2;

fn main() {
    let text = fs::read_to_string("data/input.txt").unwrap();
    // println!("{}", part2(text.clone(), 64));
    // println!("{}", part2(text.clone(), 131*2+65));
    // println!("{}", part2(text.clone(), 131*3+65));
    println!("{}", part2(text.clone(), 26501365));
}
