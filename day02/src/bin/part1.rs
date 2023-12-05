
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1},
    multi::separated_list0,
    sequence::{delimited, separated_pair, tuple},
};


fn main() {

    let input =include_str!("../../input.txt");

    let result=part1(input);
    print!("result:{}",result)

}

fn part1(input:&str) ->u32 {
    input
    .lines()
    .map(parse_line)
    .filter(|(_, batch)| is_valid(batch))
    .map(|(id, _)| id)
    .sum()



}

fn parse_line(line: &str) -> (u32, Vec<(u32, &str)>) {
    tuple::<_, _, (), _>((
        delimited(tag("Game "), complete::u32, tag(": ")),
        separated_list0(
            alt((tag("; "), tag(", "))),
            separated_pair(complete::u32, tag(" "), alpha1),
        ),
    ))(line)
    .unwrap()
    .1
}


fn is_valid(sets: &[(u32, &str)]) -> bool {
    sets.iter()
        .all(|set| matches!(set, (1..=12, "red") | (1..=13, "green") | (1..=14, "blue")))
}

