fn main() {
    let input = include_str!("../../input.txt");
    let result = part1(input);
    println!("{result}");
}

fn part1(input: &str) -> u32 {
    input.lines().map(process_line).sum()
}

fn process_line(line: &str) -> u32 {
    let first_digit = line.chars().find(char::is_ascii_digit).unwrap();
    let last_digit = line.chars().rev().find(char::is_ascii_digit).unwrap();
    format!("{first_digit}{last_digit}").parse().unwrap()
}

