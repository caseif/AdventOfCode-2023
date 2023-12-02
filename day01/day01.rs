extern crate regex;

use std::fs::{File};
use std::io::{self, BufRead};
use regex::Regex;

fn main() {
    let part_a = solve_a();
    println!("Part A: {}", part_a);

    let part_b = solve_b();
    println!("Part B: {}", part_b);
}

fn read_input_file() -> Vec<String> {
    let file = File::open("./input.txt").unwrap();
    return io::BufReader::new(file).lines()
            .filter(Result::is_ok)
            .map(Result::unwrap)
            .filter(|s| !s.is_empty())
            .collect();
}

fn parse_digit(digit: &str) -> u32 {
    if digit.len() == 1 {
        // assume it's a digit character and convert from ASCII code point
        return digit.chars().next().unwrap() as u32 - 48;
    } else {
        return match digit {
            "one" => 1,
            "two" => 2,
            "three" => 3,
            "four" => 4,
            "five" => 5,
            "six" => 6,
            "seven" => 7,
            "eight" => 8,
            "nine" => 9,
            _ => 0,
        };
    }
}

fn solve_a() -> u32 {
    let lines = read_input_file();

    let mut sum = 0;
    for line in lines {
        let first_digit = parse_digit(line.matches(char::is_numeric).next().unwrap());
        let last_digit = parse_digit(line.rmatches(char::is_numeric).next().unwrap());
        let number = first_digit * 10 + last_digit;
        sum += number;
    }

    return sum;
}

fn solve_b() -> u32 {
    let lines = read_input_file();

    let re = Regex::new("[0-9]|one|two|three|four|five|six|seven|eight|nine").unwrap();
    // We can't just get the last match because digit words can overlap.
    // Instead, reverse the string and the digit words to make sure we're
    // getting the actual last digit.
    let re_rev = Regex::new("[0-9]|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin").unwrap();

    let mut sum = 0;
    for line in lines {
        let mut fw_it = re.find_iter(line.as_str());
        let first_digit = parse_digit(fw_it.next().unwrap().as_str());

        let line_rev = line.chars().rev().collect::<String>();
        let mut rev_it = re_rev.find_iter(line_rev.as_str());
        let last_digit = parse_digit(rev_it.next().unwrap().as_str().chars().rev().collect::<String>().as_str());

        let number = first_digit * 10 + last_digit;
        sum += number;
    }

    return sum;
}
