use std::collections::HashMap;
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
            .filter(|s| !s.is_empty())
            .collect();
}

fn hash(str: &str) -> u8 {
    return str.as_bytes().iter().fold(0, |a, v| ((a + *v as u32) * 17) % 256) as u8;
}

fn solve_a() -> u32 {
    let input = &read_input_file()[0];
    return input.split(",").map(|s| hash(s) as u32).sum();
}

fn solve_b() -> u32 {
    let input = &read_input_file()[0];

    let mut bins = HashMap::<u8, Vec<(String, u8)>>::new();

    for lens in input.split(",") {
        if lens.ends_with("-") {
            let label = &lens[0..(lens.len() - 1)];

            let bin_num = hash(label);

            if !bins.contains_key(&bin_num) {
                continue;
            }

            let bin = &mut bins.get_mut(&bin_num).unwrap();
            match bin.iter().position(|(l, _)| l == label) {
                Some(i) => drop(bin.remove(i)),
                None => (),
            };
        } else {
            let (label, focus_str) = lens.split_once("=").unwrap();

            let bin_num = hash(label);

            let bin = bins.entry(bin_num).or_insert(Vec::new());
            let focus = str::parse(focus_str).unwrap();

            match bin.iter().position(|(l, _)| l == label) {
                Some(i) => bin.get_mut(i).unwrap().1 = focus,
                None => bin.push((label.to_string(), focus)),
            };
        }
    }

    let mut sum = 0;
    for (bin_num, bin) in &bins {
        for i in 0..bin.len() {
            sum += (*bin_num as u32 + 1) * (i as u32 + 1) * bin[i].1 as u32;
        }
    }

    return sum;
}
