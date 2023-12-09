
mod hands;
mod handjoker;



fn part1(input: &str) -> usize {
    let mut hands = input.lines().map(hands::Hand::from).collect::<Vec<_>>();
    hands.sort();
    let mut sum = 0;
    for (i, hand) in hands.iter().enumerate() {
        sum += hand.bet * (i + 1);
    }
    sum
}


fn main() {

    let input = include_str!("../input.txt");

    println!("Part1:{}",part1(input));

    
}