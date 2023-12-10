use std::fs;
use day9::part2;

fn main() {
    let text = fs::read_to_string("data/input.txt").unwrap();
    println!("{}", part2(text));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example2() {
        let text = fs::read_to_string("data/part2_example.txt").unwrap();
        assert_eq!(part2(text), 2)
    }
}
