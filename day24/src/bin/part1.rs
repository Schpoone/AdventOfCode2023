use day24::part1;
use std::fs;

fn main() {
    let text = fs::read_to_string("data/input.txt").unwrap();
    println!(
        "{}",
        part1(text, 200000000000000.0, 400000000000000.0)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let text = fs::read_to_string("data/part1_example.txt").unwrap();
        assert_eq!(part1(text, 7.0, 27.0), 2)
    }
}
