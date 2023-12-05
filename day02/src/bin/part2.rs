use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, digit1},
    multi::separated_list1,
    sequence::{preceded, separated_pair, tuple},
};

fn main() {
    let input = include_str!("../../input.txt");
    let result = part2(input);
    println!("{result}");
}

fn part2(input: &str) -> u32 {
    input.lines().map(parse_line).map(process_line).sum()
}

fn parse_line(line: &str) -> Vec<(u32, &str)> {
    preceded::<_, _, _, (), _, _>(
        tuple((tag("Game "), digit1, tag(": "))),
        separated_list1(
            alt((tag("; "), tag(", "))),
            separated_pair(complete::u32, tag(" "), alpha1),
        ),
    )(line)
    .unwrap()
    .1
}

fn process_line(cubes: Vec<(u32, &str)>) -> u32 {
    ["red", "green", "blue"]
        .iter()
        .map(|target_color| {
            cubes
                .iter()
                .filter(|(_, color)| color == target_color)
                .map(|(amount, _)| amount)
                .max()
                .unwrap()
        })
        .product()
}
