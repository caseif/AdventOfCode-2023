use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::BufRead;

const DIM_SIZE: usize = 140;

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

fn get_adj_cells(row: usize, col: usize) -> HashSet<(usize, usize)> {
    let mut set = HashSet::<(usize, usize)>::new();

    if row > 0 {
        set.insert((row - 1, col));
        if col > 0 {
            set.insert((row - 1, col - 1));
        }
        if col < DIM_SIZE - 1 {
            set.insert((row - 1, col + 1));
        }
    }
    if row < DIM_SIZE - 1 {
        set.insert((row + 1, col));
        if col > 0 {
            set.insert((row + 1, col - 1));
        }
        if col < DIM_SIZE - 1 {
            set.insert((row + 1, col + 1));
        }
    }
    if col > 0 {
        set.insert((row, col - 1));
    }
    if col < DIM_SIZE - 1 {
        set.insert((row, col + 1));
    }

    return set;
}

fn get_valid_cells() -> HashSet<(usize, usize)> {
    let mut valid_cells = HashSet::<(usize, usize)>::new();

    let mut row = 0;
    for line in read_input_file() {
        let mut col = 0;
        for c in line.chars() {
            if c != '.' && !c.is_ascii_digit() {
                valid_cells.extend(get_adj_cells(row, col));
            }
            col += 1;
        }
        row += 1;
    }

    return valid_cells;
}

fn solve_a() -> usize {
    let mut sum = 0;

    let valid_cells = get_valid_cells();

    let mut row = 0;
    for line in read_input_file() {
        let mut col = 0;

        let mut cur_num = 0;
        let mut save_num = false;
        for c in line.chars() {
            if c.is_ascii_digit() {
                cur_num = cur_num * 10 + (c as usize - 48);

                if !save_num && valid_cells.contains(&(row, col)) {
                    save_num = true;
                }

                if save_num && col == DIM_SIZE - 1 {
                    sum += cur_num;
                    cur_num = 0;
                    save_num = false;
                }
            } else {
                if save_num {
                    sum += cur_num;
                    save_num = false;
                }

                cur_num = 0;
            }

            col += 1;
        }

        row += 1;
    }

    return sum;
}

fn get_full_num(line: &String, row: usize, col: usize, seen: &mut HashSet<(usize, usize)>)
        -> Option<usize> {
    if seen.contains(&(row, col)) || !line.chars().nth(col).unwrap().is_ascii_digit() {
        return None;
    }

    let mut start_col: usize = col;
    while start_col > 0 && line.chars().nth(start_col - 1).unwrap().is_ascii_digit() {
        start_col -= 1;
        seen.insert((row, start_col));
    }

    let mut end_col: usize = col;
    while end_col < DIM_SIZE - 1 && line.chars().nth(end_col + 1).unwrap().is_ascii_digit() {
        end_col += 1;
        seen.insert((row, end_col));
    }

    let num_str = &line[start_col..=end_col];
    return Some(str::parse(num_str).unwrap());
}

fn get_adj_nums(row: usize, col: usize, lines: &Vec<String>) -> Vec<usize> {
    let mut nums = Vec::<usize>::new();
    let mut seen = HashSet::<(usize, usize)>::new();

    let adj_cells = get_adj_cells(row, col);
    for (adj_row, adj_col) in adj_cells {
        if lines[adj_row].chars().nth(adj_col).unwrap().is_ascii_digit() {
            if let Some(num) = get_full_num(&lines[adj_row], adj_row, adj_col, &mut seen) {
                nums.push(num);
            }
        }
    }
    return nums;
}

fn solve_b() -> usize {
    let mut sum = 0;

    let mut row = 0;
    let lines = read_input_file();
    for line in &lines {
        let mut col = 0;

        for c in line.chars() {
            if c == '*' {
                let adj_nums = get_adj_nums(row, col, &lines);
                if adj_nums.len() == 2 {
                    sum += adj_nums[0] * adj_nums[1];
                }
            }

            col += 1;
        }

        row += 1;
    }
    return sum;
}
