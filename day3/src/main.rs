use std::collections::HashSet;
use log::debug;


#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point{x, y}
    }

    fn get_all_neighbours(&self) -> Vec<Self> {
        let (x, y) = (self.x, self.y);
        vec![
            Point::new(x-1, y-1), Point::new(x, y-1), Point::new(x+1, y-1),
            self.left(), self.right(),
            Point::new(x-1, y+1), Point::new(x, y+1), Point::new(x+1, y+1),
        ]
    }

    fn left(&self) -> Self {
        Point::new(self.x - 1, self.y)
    }

    fn right(&self) -> Self {
        Point::new(self.x+1, self.y)
    }
}

struct Schematic {
    w: i32,
    h: i32,
    data: Vec<Vec<char>>,
}

impl Schematic {
    fn new(data: &str) -> Self {
        let schematic_data: Vec<Vec<char>> = data.lines().map(|line| {
            line.chars().collect()
        }).collect();
        debug!("Schematic::new with w:{}, h:{}", schematic_data[0].len(), schematic_data.len());
        debug!("Schematic data: {:?}", schematic_data);
        return Schematic {
            w: schematic_data[0].len() as i32,
            h: schematic_data.len() as i32,
            data: schematic_data,
        };
    }

    fn get_char_at(&self, point: Point) -> char {
        debug!("In get_char_at with point:{:?}", point);
        let (x, y) = (point.x, point.y);
        if x < 0 || x >= self.w || y < 0 || y >= self.h {
            return '.';
        }
        self.data[y as usize][x as usize]
    }

    fn adjacent_numbers(&self, x: usize, y: usize) -> Vec<u32> {
        let mut numbers: Vec<u32> = Vec::new();
        let origin = Point::new(x as i32, y as i32);
        let mut visited: HashSet<Point> = HashSet::new();
        for neighbour in origin.get_all_neighbours() {
            if visited.contains(&neighbour) {
                continue;
            }
            visited.insert(neighbour);
            match self.get_char_at(neighbour) {
                character if character.is_digit(10) => {
                    let mut number_start = neighbour;
                    let mut number_end = neighbour;
                    while self.get_char_at(number_start.left()).is_digit(10) {
                        number_start = number_start.left();
                        visited.insert(number_start);
                    }
                    while self.get_char_at(number_end.right()).is_digit(10) {
                        number_end = number_end.right();
                        visited.insert(number_end);
                    }
                    let number = (number_start.x..=number_end.x).map(|x| {
                        self.get_char_at(Point::new(x, neighbour.y))
                    }).collect::<String>().parse::<u32>().unwrap();
                    numbers.push(number);
                },
                _ => {} // ignore
            }

        }
        numbers
    }
}

fn part1(schematic: &Schematic) {
    println!("AOC Day 3 Part 1");
    let mut part_total: u32 = 0;
    // search across the schematic to find the non-numeric, non-. parts
    for (y, row) in schematic.data.iter().enumerate() {
        for (x, character) in row.iter().enumerate() {
            match character {
                character if character.is_digit(10) => continue,
                '.' => continue,
                _ => {
                    part_total += schematic.adjacent_numbers(x, y).iter().sum::<u32>();
                }
            }
        }
    }
    println!("Total: {}", part_total);
}

fn part2(schematic: &Schematic) {
    println!("AOC Day 3 Part 2");
    let mut gear_ratio_total: u32 = 0;
    for (y, row) in schematic.data.iter().enumerate() {
        for (x, character) in row.iter().enumerate() {
            match character {
                '*' => {
                    let adjacent_numbers = schematic.adjacent_numbers(x, y);
                    if adjacent_numbers.len() == 2 {
                        gear_ratio_total += adjacent_numbers[0] * adjacent_numbers[1];
                    }
                },
                _ => continue,
            }
        }
    }
    println!("Total: {}", gear_ratio_total);
}

fn main() {
    let data = include_str!("../input");
    let schematic = Schematic::new(data);
    part1(&schematic);
    part2(&schematic);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schematic_new() {
        let test_data = "\
...10.....
23.#......
.....741.=
.@.......3
..42...*..";
        let test_schematic = Schematic::new(&test_data);
        assert_eq!(test_schematic.w, 10);
        assert_eq!(test_schematic.h, 5);
    }

    #[test]
    fn test_schematic_get_char_at() {
        let test_data = "\
...10.....
23.#......
.....741.=
.@.......3
..42...*..";
        let test_schematic = Schematic::new(&test_data);
        assert_eq!(test_schematic.get_char_at(Point::new(1, 1)), '3');
        assert_eq!(test_schematic.get_char_at(Point::new(1, 3)), '@');
        assert_eq!(test_schematic.get_char_at(Point::new(-1, 1)), '.');
        assert_eq!(test_schematic.get_char_at(Point::new(11, 1)), '.');
    }

    #[test]
    fn test_schematic_adjacent_numbers() {
        let test_data = "\
...10.....
23.#......
.....741.=
.@.......3
..42...*..";
        let test_schematic = Schematic::new(&test_data);
        assert_eq!(test_schematic.adjacent_numbers(3, 1), [10].to_vec());
        assert_eq!(test_schematic.adjacent_numbers(9, 2), [3].to_vec());
        assert_eq!(test_schematic.adjacent_numbers(1, 3), [42].to_vec());
    }
}
