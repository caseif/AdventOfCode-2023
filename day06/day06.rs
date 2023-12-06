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

fn solve_a() -> u32 {
    let lines = read_input_file();

    let times: Vec<u32> = lines[0].split_ascii_whitespace().skip(1).map(str::parse).map(Result::unwrap).collect();
    let distances: Vec<u32> = lines[1].split_ascii_whitespace().skip(1).map(str::parse).map(Result::unwrap).collect();

    let mut races = Vec::<(u32, u32)>::new();

    for i in 0..times.len() {
        races.push((times[i], distances[i]));
    }

    let mut product = 1;
    for (time, record_dist) in races {
        let mut ways = 0;
         for hold_secs in 0..=time {
             let dist = (time - hold_secs) * hold_secs;
             if dist > record_dist {
                 ways += 1;
             }
         }

        product *= ways;
    }

    return product;
}

fn solve_b() -> u32 {
    let lines = read_input_file();

    let time: u64 = str::parse(lines[0].split_once(" ").unwrap().1.trim().replace(" ", "").as_str()).unwrap();
    let record_dist: u64 = str::parse(lines[1].split_once(" ").unwrap().1.trim().replace(" ", "").as_str()).unwrap();

    let mut ways = 0;
    for hold_secs in 0..=time {
        let dist = (time - hold_secs) * hold_secs;
        if dist > record_dist {
            ways += 1;
        }
    }

    return ways;
}
