use std::collections::HashSet;

fn part1(input: &str) {
    println!("AOC Day 4 Part 1");
    let points_total: u32 = input.lines().map(|line| {
        let (_, numbers) = line.split_once(": ").unwrap();
        let (winning_numbers, card_numbers) = numbers.split_once(" | ").unwrap();
        let winning_numbers: HashSet<&str> = HashSet::from_iter(winning_numbers.split_whitespace().collect::<Vec<&str>>().iter().cloned());
        let card_numbers: HashSet<&str> = HashSet::from_iter(card_numbers.split_whitespace().collect::<Vec<&str>>().iter().cloned());
        let result = <std::collections::hash_set::Intersection<'_, &str, _> as Iterator>::count(card_numbers.intersection(&winning_numbers)) as u32;
        if result == 0 {
            0
        } else {
            2_u32.pow(result - 1)
        }
    }).sum::<u32>();
    println!("Points total: {}", points_total);
}

fn part2(input: &str) {
    println!("AOC Day 4 Part 2");
}

fn main() {
    let data = include_str!("../input");
    part1(data);
    part2(data);
}
