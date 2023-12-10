pub fn part1(text: String) -> i64 {
    let mut sum = 0;
    for line in text.lines() {
        let mut finals = Vec::new();
        let mut cur_seq = line
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect::<Vec<i64>>();
        while !cur_seq.iter().all(|n| *n == 0) {
            let mut next_seq = Vec::new();
            for idx in 0..(cur_seq.len() - 1) {
                next_seq.push(cur_seq[idx+1] - cur_seq[idx]);
            }
            finals.push(cur_seq[cur_seq.len()-1]);
            cur_seq = next_seq;
        }
        sum += finals.iter().sum::<i64>();
    }
    sum
}

pub fn part2(text: String) -> i64 {
    let mut sum = 0;
    for line in text.lines() {
        let mut firsts = Vec::new();
        let mut cur_seq = line
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect::<Vec<i64>>();
        while !cur_seq.iter().all(|n| *n == 0) {
            let mut next_seq = Vec::new();
            for idx in 0..(cur_seq.len() - 1) {
                next_seq.push(cur_seq[idx+1] - cur_seq[idx]);
            }
            firsts.push(cur_seq[0]);
            cur_seq = next_seq;
        }
        sum += firsts.iter().rev().fold(0, |acc, n| n - acc);
    }
    sum
}
