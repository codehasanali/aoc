fn main() {
    let input = include_str!("../../input.txt");
    let result = part2(input);
    println!("{result}");
}

const DIGITS: [&str; 18] = [
    "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
    "seven", "eight", "nine",
];

fn part2(input: &str) -> u32 {
    input.lines().map(process_line).sum()
}

fn process_line(line: &str) -> u32 {
    let first_digit = (0..line.len())
        .find_map(|start| {
            DIGITS
                .iter()
                .position(|&digit| line[start..].starts_with(digit))
        })
        .map(|index| DIGITS[index % 9])
        .unwrap();
    let last_digit = (0..line.len())
        .rev()
        .find_map(|end| {
            DIGITS
                .iter()
                .position(|&digit| line[..=end].ends_with(digit))
        })
        .map(|index| DIGITS[index % 9])
        .unwrap();
    format!("{first_digit}{last_digit}").parse().unwrap()
}

