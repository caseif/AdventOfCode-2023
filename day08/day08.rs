use std::cmp::min;
use std::collections::HashMap;
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

fn gcd(a: u64, b: u64) -> u64 {
    if a == 0 {
        return b;
    } else if b == 0 {
        return a;
    } else if a % 2 == 0 && b % 2 == 0 {
        return 2 * gcd(a / 2, b / 2);
    } else if a % 2 == 0 {
        return gcd(a / 2, b);
    } else if b % 2 == 0 {
        return gcd(a, b / 2);
    } else {
        return gcd(a.abs_diff(b), min(a, b));
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    return a / gcd(a, b) * b;
}

fn solve() -> (u32, u64) {
    let lines = read_input_file();

    let steps = lines[0].chars().collect::<Vec<char>>();

    let mut nodes = HashMap::<&str, (&str, &str)>::new();

    for line in lines.iter().skip(1) {
        let (node, lr) = line.split_once(" = ").unwrap();
        let (l, r) = lr[1..9].split_once(", ").unwrap();
        nodes.insert(node, (l, r));
    }

    let start_nodes = nodes.keys().into_iter().filter(|n| n.ends_with("A")).cloned().collect::<Vec<&str>>();

    let mut ans_a = 0u32;
    let mut ans_b = 1u64;

    for start_node in start_nodes {
        let mut cur_node = start_node;

        let mut cur_step = 0;
        let mut total_steps = 0;

        while !cur_node.ends_with("Z") {
            total_steps += 1;

            let step = steps[cur_step];

            let (l, r) = nodes[cur_node];
            cur_node = match step {
                'L' => l,
                'R' => r,
                _ => panic!(),
            };

            if start_node == "AAA" && cur_node == "ZZZ" {
                ans_a = total_steps;
            }

            cur_step = (cur_step + 1) % steps.len();
        }

        ans_b = lcm(ans_b, total_steps as u64);
    }

    return (ans_a, ans_b);
}
