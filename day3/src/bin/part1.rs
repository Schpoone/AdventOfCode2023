use day3::part1;
use std::fs;

fn main() {
    let text = fs::read_to_string("data/input.txt").unwrap();
    println!("{}", part1(text));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let text = fs::read_to_string("data/part1_example.txt").unwrap();
        assert_eq!(part1(text), 4361)
    }
}
