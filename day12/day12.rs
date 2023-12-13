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

fn is_valid_config(groups: &Vec<u64>, conds: &str) -> bool {
    let prefix = if conds.contains("?") {
        conds.split_once("?").unwrap().0
    } else {
        conds
    };

    let actual_groups: Vec<u64> = prefix
            .split(".")
            .filter(|s| !s.is_empty())
            .map(|s| s.len() as u64)
            .collect();

    if actual_groups.len() > groups.len() {
        return false;
    }

    if !conds.contains("?") {
        if groups.as_slice() != actual_groups {
        }
        return groups.as_slice() == actual_groups;
    }

    let last_actual = if actual_groups.len() > 0 { actual_groups[actual_groups.len() - 1] } else { 0 };
    let last_expected = if actual_groups.len() > 0 { groups[actual_groups.len() - 1] } else { 0 };
    if last_actual > last_expected {
        return false;
    }

    let max_broken = conds.replace(".", "").len() as u64;
    let total_expected_broken: u64 = groups.iter().sum();

    if max_broken < total_expected_broken {
        return false;
    }

    let remaining_broken = last_expected
            - last_actual
            + groups[actual_groups.len()..].iter().sum::<u64>();
    if ((conds.len() - prefix.len() + 1) as u64) < remaining_broken {
        return false;
    }

    return true;
}

fn get_valid_configs(groups: &Vec<u64>, conds: String, seen: &mut HashMap<(Vec<u64>, String), u64>) -> u64 {
    let key = (groups.clone(), conds.clone());
    if seen.contains_key(&key) {
        return seen[&key];
    }


    if !is_valid_config(groups, &conds) {
        seen.insert(key, 0);
        return 0;
    }


    let prefix = if conds.contains("?") {
        &conds[0..conds.split_once("?").unwrap().0.rfind('.').unwrap_or(0)]
    } else {
        &conds
    };

    let actual_groups: Vec<u64> = prefix
            .split(".")
            .filter(|s| !s.is_empty())
            .map(|s| s.len() as u64)
            .collect();

    if actual_groups.len() > 0
            && (actual_groups.len() > groups.len()
                    || actual_groups != groups[0..actual_groups.len()]) {
        seen.insert(key, 0);
        return 0;
    }

    if actual_groups.len() == groups.len() {
        if actual_groups != groups.as_slice()
                || conds[prefix.len()..].contains("#") {
            seen.insert(key, 0);
            return 0;
        } else {
            seen.insert(key, 1);
            return 1;
        }
    }

    let new_groups = groups[actual_groups.len()..groups.len()].to_vec();
    let new_conds = &conds[prefix.len()..];

    if new_conds.contains("?") {
        let with_working = new_conds.replacen("?", ".", 1);
        let with_broken = new_conds.replacen("?", "#", 1);
        let total = get_valid_configs(&new_groups, with_working, seen)
                + get_valid_configs(&new_groups, with_broken, seen);
        seen.insert(key, total);
        return total;
    } else {
        let res = if actual_groups == groups.as_slice() { 1 } else { 0 };
        seen.insert(key, res);
        return res;
    }
}

fn solve() -> (u64, u64) {
    let lines = read_input_file();

    let mut sum_a = 0;
    let mut sum_b: u64 = 0;

    for line in lines {
        let (conds_a, groups_str_a) = line.split_once(" ").unwrap();

        let conds_b = (0..5).map(|_| conds_a).collect::<Vec<&str>>().join("?");
        let groups_str_b = (0..5).map(|_| groups_str_a).collect::<Vec<&str>>().join(",");

        let groups_a: Vec<u64> = groups_str_a.split(",").map(|s| str::parse::<u64>(s).unwrap()).collect();
        let groups_b: Vec<u64> = groups_str_b.split(",").map(|s| str::parse::<u64>(s).unwrap()).collect();

        let valid_configs_a = get_valid_configs(&groups_a, conds_a.to_string(), &mut HashMap::new());
        sum_a += valid_configs_a;

        let valid_configs_b = get_valid_configs(&groups_b, conds_b.to_string(), &mut HashMap::new());
        sum_b += valid_configs_b;
    }

    return (sum_a, sum_b);
}
