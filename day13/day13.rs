use std::fs::File;
use std::io;
use std::io::BufRead;

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
            .collect();
}

fn transpose_pattern(pattern: &[String]) -> Vec<String> {
    return (0..pattern[0].len())
            .map(|i| pattern.iter()
                    .map(|line| line.chars().nth(i).unwrap())
                    .collect::<String>())
            .collect();
}

fn is_power_of_two(x: u32) -> bool {
    return (x & (x - 1)) == 0;
}

fn find_refl_line(pattern: &[String]) -> Option<u32> {
    for i in 0..(pattern.len() - 1) {
        let mut valid = true;
        for j in 0..=i {
            if i + j + 1 >= pattern.len() {
                break;
            }
            if &pattern[i - j] != &pattern[i + j + 1] {
                valid = false;
                break;
            }
        }

        if !valid {
            continue;
        }

        return Some(i as u32 + 1);
    }

    return None;
}

fn find_smudged_refl_line(pattern: &[String]) -> Option<u32> {
    let old_refl_line = find_refl_line(pattern);

    let pattern_bits: Vec<u32> = pattern.iter()
            .map(|line| line.chars().enumerate()
                    .filter(|(_, c)| *c == '#')
                    .map(|(i, _)| 1u32 << i)
                    .sum())
            .collect();
    //println!("{:?}", pattern_bits);

    for i in 0..(pattern.len() - 1) {
        let mut used_smudge = false;

        if old_refl_line.is_some() && i as u32 + 1 == old_refl_line.unwrap() {
            continue;
        }

        let mut valid = true;
        for j in 0..=i {
            if i + j + 1 >= pattern.len() {
                break;
            }

            if &pattern[i - j] != &pattern[i + j + 1] {
                if !used_smudge && is_power_of_two(pattern_bits[i - j] ^ pattern_bits[i + j + 1]) {
                    used_smudge = true;
                } else {
                    valid = false;
                    break;
                }
            }
        }

        if !valid || (!used_smudge && (pattern.len() % 2) == 0 && i == pattern.len() / 2) {
            continue;
        }

        if !valid {
            continue;
        }

        return Some(i as u32 + 1);
    }

    return None;
}

fn solve_a() -> u32 {
    let lines = read_input_file();

    let mut sum = 0;

    for pattern in lines.split(|l| l.is_empty()) {
        let transposed = transpose_pattern(pattern);

        if let Some(refl) = find_refl_line(pattern) {
            sum += refl * 100;
        } else if let Some(refl) = find_refl_line(transposed.as_slice()) {
            sum += refl;
        } else {
            panic!("No reflection found");
        }
    }

    return sum;
}

fn solve_b() -> u32 {
    let lines = read_input_file();

    let mut sum = 0;

    for pattern in lines.split(|l| l.is_empty()) {
        /*println!("pattern:");
        for line in pattern {
            println!("{line}");
        }
        println!();*/
        let transposed = transpose_pattern(pattern);

        if let Some(refl) = find_smudged_refl_line(pattern) {
            sum += refl * 100;
        } else if let Some(refl) = find_smudged_refl_line(transposed.as_slice()) {
            sum += refl;
        } else {
            panic!("No reflection found");
        }
    }

    return sum;
}
