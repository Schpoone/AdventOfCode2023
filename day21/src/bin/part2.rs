use std::fs;
use day21::part2;

fn main() {
    let text = fs::read_to_string("data/input.txt").unwrap();
    println!("{}", part2(text, 26501365));
    // let text = fs::read_to_string("data/part2_example.txt").unwrap();
    // println!("{}", part2(text, 210));
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test with the new example that has rocks cleared in the cardinal directions

    #[test]
    fn example2() {
        let text = fs::read_to_string("data/part2_example.txt").unwrap();
        assert_eq!(part2(text, 6), 36)
    }
    #[test]
    fn example3() {
        let text = fs::read_to_string("data/part2_example.txt").unwrap();
        assert_eq!(part2(text, 10), 90)
    }
    #[test]
    fn example4() {
        let text = fs::read_to_string("data/part2_example.txt").unwrap();
        assert_eq!(part2(text, 50), 1940)
    }
    #[test]
    fn example5() {
        let text = fs::read_to_string("data/part2_example.txt").unwrap();
        assert_eq!(part2(text, 100), 7645)
    }
    #[test]
    fn example6() {
        let text = fs::read_to_string("data/part2_example.txt").unwrap();
        assert_eq!(part2(text, 500), 188756)
    }
    #[test]
    fn example7() {
        let text = fs::read_to_string("data/part2_example.txt").unwrap();
        assert_eq!(part2(text, 1000), 753480)
    }
    #[test]
    fn example8() {
        let text = fs::read_to_string("data/part2_example.txt").unwrap();
        assert_eq!(part2(text, 5000), 18807440)
    }
}
