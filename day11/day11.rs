use std::cmp::{max, min};
use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::BufRead;

fn main() {
    let (part_a, part_b) = solve();
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

fn solve() -> (u32, u64) {
    let lines = read_input_file();

    let galaxies: Vec<(u32, u32)> = lines.iter().enumerate()
            .map(|(row, line)| line.chars().enumerate()
                    .filter(|(_, c)| *c == '#')
                    .map(|(col, _)| (col as u32, row as u32))
                    .collect::<Vec<(u32, u32)>>())
            .flatten()
            .collect();
    let present_rows: HashSet<u32> = galaxies.iter().map(|(_, y)| *y).collect();
    let present_cols: HashSet<u32> = galaxies.iter().map(|(x, _)| *x).collect();
    let empty_rows: HashSet<u32> = (0..(lines.len() as u32)).filter(|i| !present_rows.contains(i)).collect();
    let empty_cols: HashSet<u32> = (0..(lines[0].len() as u32)).filter(|i| !present_cols.contains(i)).collect();

    let mut sum_a = 0;
    let mut sum_b = 0;

    for i in 0..galaxies.len() {
        for j in (i + 1)..galaxies.len() {
            let (x1, y1) = galaxies[i];
            let (x2, y2) = galaxies[j];
            let x_min = min(x1, x2);
            let x_max = max(x1, x2);
            let y_min = min(y1, y2);
            let y_max = max(y1, y2);

            let extra_rows = empty_rows.iter().filter(|row| **row > y_min && **row < y_max).count() as u32;
            let extra_cols = empty_cols.iter().filter(|col| **col > x_min && **col < x_max).count() as u32;
            let base_dist = (x_max - x_min) + (y_max - y_min);
            let dist_a = base_dist + extra_rows + extra_cols;
            let dist_b = base_dist as u64 + (extra_rows + extra_cols) as u64 * 999999;
            sum_a += dist_a;
            sum_b += dist_b;
        }
    }

    return (sum_a, sum_b);
}
