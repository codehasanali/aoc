struct Readings {
    pub data: Vec<i32>,
}

impl Readings {
    pub fn parse(line: &str) -> Self {
        Self {
            data: line
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect(),
        }
    }
    pub fn predict_after(&self) -> i32 {
        fn predict_rec(n: &[i32]) -> i32 {
            let reduced = n.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();
            if reduced.iter().all(|&n| n == 0) {
                return *n.last().unwrap();
            }
            let rec = predict_rec(&reduced);
            n.last().unwrap() + rec
        }
        predict_rec(&self.data)
    }
    pub fn predict_before(&self) -> i32 {
        fn predict_rec(n: &[i32]) -> i32 {
            let reduced = n.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();
            if reduced.iter().all(|&n| n == 0) {
                return *n.first().unwrap();
            }
            let rec = predict_rec(&reduced);
            n.first().unwrap() - rec
        }
        predict_rec(&self.data)
    }
}

fn main() {
    let input = include_str!("../input.txt");

    println!("part1:{}", part1(input));

    println!("part2:{}",part2(input));
}

fn part1(input: &str) -> i32 {
    input
        .lines()
        .map(Readings::parse)
        .map(|r| r.predict_after())
        .sum()
}
fn part2(input: &str) -> i32 {
    input
        .lines()
        .map(Readings::parse)
        .map(|r| r.predict_before())
        .sum()
}