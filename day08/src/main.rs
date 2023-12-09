use std::collections::HashMap;

use num::integer::lcm;

enum Direction {
    Left,
    Right,
}
impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Invalid direction"),
        }
    }
}
struct Map<'a> {
    pub network: HashMap<&'a str, (&'a str, &'a str)>,
    pub directions: Vec<Direction>,
}
impl<'a> Map<'a> {
    pub fn parse(input: &'a str) -> Self {
        let mut lines = input.lines();
        let directions = lines
            .next()
            .unwrap()
            .chars()
            .map(Direction::from)
            .collect::<Vec<_>>();
        // let network = HashMap::new();
        let network = lines
            .skip(1)
            .map(|s| {
                let parts = s.split_once('=').unwrap();
                let map = parts.1.split_once(',').unwrap();
                (
                    parts.0.trim(),
                    (
                        map.0.trim_matches([' ', '('].as_slice()),
                        map.1.trim_matches([' ', ')'].as_slice()),
                    ),
                )
            })
            .collect();
        Map {
            network,
            directions,
        }
    }
}


fn part1(input: &str) -> usize {
    let map = Map::parse(input);
    let mut current: &str = "AAA";
    let mut directions = map.directions.iter().cycle();
    let mut count: usize = 0;
    while current != "ZZZ" {
        let node = map.network.get(current).unwrap();
        count += 1;
        match directions.next().unwrap() {
            Direction::Left => current = node.0,
            Direction::Right => current = node.1,
        }
    }
    count
}




fn main() {

    let input =include_str!("../input.txt");

    println!("Part1:{}",part1(input));


}
