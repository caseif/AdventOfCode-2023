use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap};
use std::fs::File;
use std::hash::{Hash, Hasher};
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
            .filter(|s| !s.is_empty())
            .collect();
}

fn transpose_strings(strings: &[String]) -> Vec<String> {
    return (0..strings[0].len())
            .map(|i| strings.iter()
                    .map(|line| line.chars().nth(i).unwrap())
                    .collect::<String>())
            .collect();
}

fn solve_a() -> u32 {
    let lines = read_input_file();
    let cols = transpose_strings(lines.as_slice());

    let mut total_load = 0u32;

    for col in cols {
        let mut next_load = col.len();
        for i in 0..col.len() {
            match col.chars().nth(i).unwrap() {
                '.' => (),
                '#' => next_load = col.len() - i - 1,
                'O' =>  {
                    total_load += next_load as u32;
                    next_load -= 1;
                },
                _ => panic!(),
            };
        }
    }

    return total_load;
}

fn rotate_cw(strs: &Vec<String>) -> Vec<String> {
    return transpose_strings(strs).iter().map(|s| s.chars().rev().collect()).collect();
}

fn rotate_ccw(strs: &Vec<String>) -> Vec<String> {
    return transpose_strings(strs.iter().map(|s| s.chars().rev().collect()).collect::<Vec<String>>().as_slice());
}

fn do_roll(strs: &Vec<String>) -> Vec<String> {
    return strs.iter().map(|line|
        line.split('#').map(|s| {
            let num_rocks = s.matches('O').count();
            "O".repeat(num_rocks) + ".".repeat(s.len() - num_rocks).as_str()
        }).collect::<Vec<String>>().join("#")).collect();
}

fn do_spin(rows: &Vec<String>) -> Vec<String> {
    let mut strs = rows.clone();
    for _ in 0..4 {
        strs = rotate_cw(&do_roll(&strs));
    }

    return strs;
}

fn solve_b() -> u32 {
    let mut hash_indices = HashMap::<u64, u32>::new();
    let mut index_states = HashMap::<u32, Vec<String>>::new();

    let mut rows = rotate_ccw(&read_input_file());

    let mut first_repeat: u32 = 0;
    let mut second_repeat: u32 = 0;

    for i in 1..=1000000000 {
        rows = do_spin(&rows);

        let mut hasher = DefaultHasher::new();
        rows.hash(&mut hasher);
        let hash = hasher.finish();
        if hash_indices.contains_key(&hash) {
            if first_repeat == 0 {
                first_repeat = i;
                hash_indices.clear();
                hash_indices.insert(hash, i);
                index_states.insert(i, rows.clone());
            } else {
                second_repeat = i;
                break;
            }
        } else {
            hash_indices.insert(hash, i);
            index_states.insert(i, rows.clone());
        }
    }

    let repeat_interval = second_repeat - first_repeat;
    let res_offset = (1000000000 - first_repeat) % repeat_interval;

    let state = &index_states[&(first_repeat + res_offset)];

    let mut total_load = 0u32;
    for row in state {
        for i in 0..row.len() {
            if row.chars().nth(i).unwrap() == 'O' {
                total_load += (row.len() - i) as u32;
            }
        }
    }

    return total_load;
}
