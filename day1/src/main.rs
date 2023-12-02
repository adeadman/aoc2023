use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use phf::phf_map;
use log::debug;

static NUMBERS: phf::Map<&'static str, char> = phf_map! {
    "zero" => '0',
    "0" => '0',
    "one" => '1',
    "1" => '1',
    "two" => '2',
    "2" => '2',
    "three" => '3',
    "3" => '3',
    "four" => '4',
    "4" => '4',
    "five" => '5',
    "5" => '5',
    "six" => '6',
    "6" => '6',
    "seven" => '7',
    "7" => '7',
    "eight" => '8',
    "8" => '8',
    "nine" => '9',
    "9" => '9',
};


fn part1() -> io::Result<()> {
    println!("AOC Day 1 Part 1");
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut total: i32 = 0;
    for line in reader.lines() {
        let mut digits = Vec::new();
        for character in line?.chars() {
            if character.is_numeric() {
                digits.push(character);
            }
        }
        let digit_string: String = format!("{}{}",
            digits.get(0).unwrap(),
            digits.get(digits.len().wrapping_sub(1)).unwrap());
        total += digit_string.parse::<i32>().unwrap();
    }
    println!("Total is {}", total);
    Ok(())
}

fn get_digit_string(line: String) -> String {
    let mut line = line;
    let mut digits: Vec<char> = Vec::new();
    debug!("Line is {}", line);
    while line.len() > 0 {
        if let Some(number) = NUMBERS.keys().find(|number| line.starts_with(*number)) {
            digits.push(*(NUMBERS.get(&number as &str).unwrap()));
        }
        line = line[1..].to_string();
    }

    debug!("Digits are {:?}", digits);
    format!("{}{}",
        digits.get(0).unwrap(),
        digits.get(digits.len().wrapping_sub(1)).unwrap())
}

fn part2() -> io::Result<()> {
    println!("AOC Day 1 Part 2");
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut total: i32 = 0;
    for line in reader.lines() {
        let digit_string: String = get_digit_string(line?);
        debug!("Digit string is {}", digit_string);
        total += digit_string.parse::<i32>().unwrap();
    }
    println!("Total is {}", total);
    Ok(())
}


fn main() -> io::Result<()> {
    part1()?;
    part2()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_digit_string_easy() {
        assert_eq!(get_digit_string("2eightsix16".to_string()), "26".to_string());
    }

    #[test]
    fn test_get_digit_string_medium() {
        assert_eq!(get_digit_string("19qdlpmdrxone7sevennine".to_string()), "19".to_string());
    }

    #[test]
    fn test_get_digit_string_hard() {
        assert_eq!(get_digit_string("cdtjprrbvkftgtwo397seven".to_string()), "27".to_string());
    }
}
