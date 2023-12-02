use std::fs;
use day1::{part1, part2};

fn main() {
    let text = fs::read_to_string("data/part1_input.txt").unwrap();
    println!("{}", part1(text));

    let text = fs::read_to_string("data/part2_input.txt").unwrap();
    println!("{}", part2(text));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let text = fs::read_to_string("data/part1_example.txt").unwrap();
        assert_eq!(part1(text), )
    }

    #[test]
    fn example2() {
        let text = fs::read_to_string("data/part2_example.txt").unwrap();
        assert_eq!(part1(text), )
    }
}
