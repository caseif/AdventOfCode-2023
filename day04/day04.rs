use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::iter::FromIterator;

fn main() {
    let part_a = solve_a();
    let part_b = solve_b();
    println!("Part A: {}", part_a);
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

fn solve_a() -> u32 {
    let mut total: u32 = 0;
    for line in read_input_file() {
        let card_str = &line[9..];
        let (win_str, my_str) = card_str.split_once(" | ").unwrap();
        let winning_nums: HashSet<&str> = HashSet::from_iter(win_str.trim().split(" ").filter(|s| !s.is_empty()));
        let my_nums: HashSet<&str> = HashSet::from_iter(my_str.trim().split(" ").filter(|s| !s.is_empty()));

        let win_count = winning_nums.intersection(&my_nums).count();
        total += if win_count > 0 {
            2_u32.pow(win_count as u32 - 1)
        } else {
            0
        };
    }

    return total;
}

fn solve_b() -> u32 {
    let mut total: u32 = 0;
    let lines = read_input_file();
    let count = lines.iter().count() as u32;
    let mut multipliers = HashMap::<u32, u32>::new();

    let mut i = 1_u32;
    for line in read_input_file() {
        let multiplier = multipliers.get(&i).unwrap_or(&1).clone();

        let card_str = &line[9..];
        let (win_str, my_str) = card_str.split_once(" | ").unwrap();
        let winning_nums: HashSet<&str> = HashSet::from_iter(win_str.trim().split(" ").filter(|s| !s.is_empty()));
        let my_nums: HashSet<&str> = HashSet::from_iter(my_str.trim().split(" ").filter(|s| !s.is_empty()));

        let win_count = winning_nums.intersection(&my_nums).count() as u32;
        for j in (i + 1)..(i + 1 + win_count) {
            if j <= count {
                *multipliers.entry(j).or_insert(1) += multiplier;
            }
        }

        total += multiplier;
        i += 1;
    }

    return total;
}
