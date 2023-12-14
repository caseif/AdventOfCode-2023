use std::cmp::max;
use std::fs::File;
use std::io;
use std::io::BufRead;

const RED_COUNT: u32 = 12;
const GREEN_COUNT: u32 = 13;
const BLUE_COUNT: u32 = 14;

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

fn solve() -> (u32, u32) {
    let mut part_a = 0;
    let mut part_b = 0;

    let mut game = 1;
    for line in read_input_file() {
        let mut impossible = false;

        let game_sets = line.split(": ").nth(1).unwrap().split("; ")
                .map(|s| s.split(", ")
                        .map(|s| s.split(" "))
                        .map(|mut spl| (spl.next().unwrap(), spl.next().unwrap()))
                        .collect::<Vec<(&str, &str)>>())
                .collect::<Vec<Vec<(&str, &str)>>>();

        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        for set in game_sets {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;

            for (val, color) in set {
                let num = str::parse::<u32>(val).unwrap();
                match color {
                    "red" => red += num,
                    "green" => green += num,
                    "blue" => blue += num,
                    _ => panic!(),
                };
            }

            if red > RED_COUNT || green > GREEN_COUNT || blue > BLUE_COUNT {
                impossible = true;
            }

            max_red = max(max_red, red);
            max_green = max(max_green, green);
            max_blue = max(max_blue, blue);
        }

        if !impossible {
            part_a += game;
        }

        part_b += max_red * max_green * max_blue;

        game += 1;
    }

    return (part_a, part_b);
}
