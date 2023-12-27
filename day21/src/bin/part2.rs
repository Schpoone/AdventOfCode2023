use std::fs;
use day21::part2;

fn main() {
    let text = fs::read_to_string("data/input.txt").unwrap();
    // let text = fs::read_to_string("data/part2_example.txt").unwrap();
    // println!("{}", part2(text, 26501365));
    println!("{}", part2(text, 210));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example2() {
        let text = fs::read_to_string("data/part2_example.txt").unwrap();
        assert_eq!(part2(text, 6), 16)
    }
    #[test]
    fn example3() {
        let text = fs::read_to_string("data/part2_example.txt").unwrap();
        assert_eq!(part2(text, 10), 50)
    }
    #[test]
    fn example4() {
        let text = fs::read_to_string("data/part2_example.txt").unwrap();
        assert_eq!(part2(text, 50), 1594)
    }
    #[test]
    fn example5() {
        let text = fs::read_to_string("data/part2_example.txt").unwrap();
        assert_eq!(part2(text, 100), 6536)
    }
    #[test]
    fn example6() {
        let text = fs::read_to_string("data/part2_example.txt").unwrap();
        assert_eq!(part2(text, 500), 167004)
    }
    #[test]
    fn example7() {
        let text = fs::read_to_string("data/part2_example.txt").unwrap();
        assert_eq!(part2(text, 1000), 668697)
    }
    #[test]
    fn example8() {
        let text = fs::read_to_string("data/part2_example.txt").unwrap();
        assert_eq!(part2(text, 5000), 16733044)
    }
}
