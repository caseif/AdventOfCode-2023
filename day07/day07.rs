use std::cmp::Ordering;
use std::collections::HashMap;
use std::convert::TryInto;
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

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
enum HandType {
    Junk,
    Pair,
    TwoPair,
    Three,
    FullHouse,
    Four,
    Five
}

#[derive(Debug)]
struct Hand {
    hand_type: HandType,
    hand: [char; 5],
    with_wild: bool,
}

fn get_char_val(c: char, wild: bool) -> u32 {
    return match c {
        'A' => 14u32,
        'K' => 13u32,
        'Q' => 12u32,
        'J' => if wild { 1u32 } else { 11u32 },
        'T' => 10u32,
        _ => str::parse::<u32>(&c.to_string()).unwrap(),
    };
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        return self.hand == other.hand;
    }
}

impl Eq for Hand {
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.hand_type != other.hand_type {
            return Some(self.hand_type.cmp(&other.hand_type));
        } else {
            for i in 0..5 {
                if self.hand[i] != other.hand[i] {
                    return Some(get_char_val(self.hand[i], self.with_wild)
                            .cmp(&get_char_val(other.hand[i], self.with_wild)));
                }
            }
            panic!("Failed to compare hands");
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        return self.partial_cmp(other).unwrap();
    }
}

fn parse_hand(hand: &str, with_wild: bool) -> Hand {
    let chars = hand.chars().collect::<Vec<char>>();

    let mut char_counts: HashMap<char, u32> = HashMap::new();
    chars.iter().for_each(|c| *char_counts.entry(*c).or_insert(0) += 1);
    let mut counts: Vec<u32> = char_counts.iter().filter(|(c, _)| !with_wild || *c != &'J').map(|(_, v)| *v).collect();
    counts.sort();
    counts.reverse();

    if with_wild {
        let num_wild = chars.iter().filter(|c| *c == &'J').count() as u32;
        if num_wild == 5 {
            counts.push(5);
        } else {
            counts[0] += num_wild;
        }
    }

    let hand_type = match counts.as_slice() {
        [5] => HandType::Five,
        [4, 1] => HandType::Four,
        [3, 2] => HandType::FullHouse,
        [3, 1, 1] => HandType::Three,
        [2, 2, 1] => HandType::TwoPair,
        [2, 1, 1, 1] => HandType::Pair,
        _ => HandType::Junk,
    };

    return Hand { hand_type, hand: hand.chars().collect::<Vec<char>>()[0..5].try_into().unwrap(), with_wild };
}

fn solve(with_wild: bool) -> u32 {
    let lines = read_input_file();
    let mut hands: Vec<(Hand, u32)> = lines.iter().map(|s| s.split_once(" ").unwrap()).map(|h| {
        (parse_hand(h.0, with_wild), str::parse::<u32>(h.1).unwrap())
    }).collect();

    hands.sort_by(|(h1, _), (h2, _)| h1.cmp(h2));

    let sum = hands.iter().enumerate().map(|(i, (_, bet))| (i as u32 + 1) * bet).sum();
    return sum;
}

fn solve_a() -> u32 {
    solve(false)
}

fn solve_b() -> u32 {
    solve(true)
}
