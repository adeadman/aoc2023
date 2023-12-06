use std::collections::{HashMap, HashSet};

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
    let mut scratchcards = HashMap::<u32, u32>::new();
    for line in input.lines() {
        let (card, numbers) = line.split_once(": ").unwrap();
        let current_card_number = card.split_whitespace().collect::<Vec<_>>()[1].parse::<u32>().unwrap();
        scratchcards.entry(current_card_number).and_modify(|e| {*e += 1}).or_insert(1);
        let number_current_scratchcards = scratchcards[&current_card_number];
        let (winning_numbers, card_numbers) = numbers.split_once(" | ").unwrap();
        let winning_numbers: HashSet<&str> = HashSet::from_iter(winning_numbers.split_whitespace().collect::<Vec<&str>>().iter().cloned());
        let card_numbers: HashSet<&str> = HashSet::from_iter(card_numbers.split_whitespace().collect::<Vec<&str>>().iter().cloned());
        let matches = <std::collections::hash_set::Intersection<'_, &str, _> as Iterator>::count(card_numbers.intersection(&winning_numbers)) as u32;
        if matches > 0 {
            for card_number in (current_card_number+1)..=(current_card_number+matches) {
                scratchcards.entry(card_number).and_modify(|e| {
                    *e += number_current_scratchcards
                }).or_insert(number_current_scratchcards);
            }
        }
    }
    let total_scratchcards: u32 = scratchcards.values().cloned().collect::<Vec<u32>>().iter().sum();
    println!("Total: {}", total_scratchcards);
}

fn main() {
    let data = include_str!("../input");
    part1(data);
    part2(data);
}
