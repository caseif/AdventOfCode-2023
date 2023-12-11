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

fn solve() -> (i32, i32) {
    let lines = read_input_file();

    let mut sum_a: i32 = 0;
    let mut sum_b: i32 = 0;

    for line in lines {
        let orig_vals = line.split(" ").map(|s| str::parse(s).unwrap()).collect::<Vec<i32>>();
        let mut vals = orig_vals.clone();
        let mut val_sets = Vec::<Vec<i32>>::new();
        val_sets.push(orig_vals);

        while vals.iter().any(|v| *v != vals[0]) {
            let mut new_vals = Vec::<i32>::new();
            for i in 0..(vals.len() - 1) {
                new_vals.push(vals[i + 1] - vals[i]);
            }
            vals = new_vals.clone();
            val_sets.insert(0, new_vals);
        }

        for i in 1..val_sets.len() {
            let prev_set = &val_sets[i - 1];
            let prev_first = prev_set[0];
            let prev_last = prev_set[prev_set.len() - 1];

            let cur_set = &mut val_sets[i];
            let first = cur_set[0];
            let last = cur_set[cur_set.len() - 1];

            cur_set.insert(0, first - prev_first);
            cur_set.push(last + prev_last);
        }

        let last_set = &val_sets[val_sets.len() - 1];
        let left_val = last_set[0];
        let right_val = last_set[last_set.len() - 1];

        sum_a += right_val;
        sum_b += left_val;
    }

    return (sum_a, sum_b);
}
