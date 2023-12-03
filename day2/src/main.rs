use std::cmp::max;
use phf::phf_map;

static MAX_COLOURS: phf::Map<&'static str, i32> = phf_map! {
    "red" => 12,
    "green" => 13,
    "blue" => 14,
};

fn is_impossible(turn: &str) -> bool {
    turn.split(", ").any(|colour_group| {
        let (number, colour) = colour_group.split_once(" ").unwrap();
        let number = number.parse::<i32>().unwrap();
        number > MAX_COLOURS[colour]
    })
}

fn part1(input: &str) {
    println!("AOC Day 2 Part 1");

    let total: u32 = input.lines().filter_map(|line| {
        let (game, turns) = line.split_once(": ").unwrap();
        let game_id = game.strip_prefix("Game ").unwrap().parse::<u32>().unwrap();
        let is_valid = turns.split("; ").all(|set| {
            !is_impossible(set)
        });
        if is_valid {Some(game_id)} else {None}
    }).sum::<u32>();
    println!("Total is {}", total);
}

fn part2(input: &str) {
    println!("AOC Day 2 Part 2");

    let total: u32 = input.lines().map(|line| {
        let (_, turns) = line.split_once(": ").unwrap();
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;
        for turn in turns.split("; ") {
            for colour_group in turn.split(", ") {
                let (number, colour) = colour_group.split_once(" ").unwrap();
                let number = number.parse::<u32>().unwrap();
                match colour {
                    "red" => min_red = max(min_red, number),
                    "green" => min_green = max(min_green, number),
                    "blue" => min_blue = max(min_blue, number),
                    _ => panic!("Unexpected colour"),
                }
            }
        }
        min_red * min_green * min_blue
    }).sum::<u32>();
    println!("Total is {}", total);
}


fn main() {
    let input = include_str!("../input");
    part1(input);
    part2(input);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_impossible_false() {
        assert_eq!(is_impossible("8 red, 7 green, 10 blue"), false);
    }

    #[test]
    fn test_is_impossible_true() {
        assert_eq!(is_impossible("13 red, 4 green"), true);
    }
}
