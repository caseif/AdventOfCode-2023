use std::collections::{HashSet, VecDeque};
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Pipe {
    None,
    Start,
    Vertical,
    Horizontal,
    LUBend,
    LDBend,
    RUBend,
    RDBend,
}

impl Pipe {
    fn move_through(&self, last_dir: &Direction) -> Option<Direction> {
        let coming_from = last_dir.inverse();
        match self {
            Pipe::None => panic!("Cannot move from empty position"),
            Pipe::Start => panic!("Cannot move from start position"),
            Pipe::Vertical => match coming_from {
                Direction::Up => Some(Direction::Down),
                Direction::Down => Some(Direction::Up),
                _ => None,
            },
            Pipe::Horizontal => match coming_from {
                Direction::Right => Some(Direction::Left),
                Direction::Left => Some(Direction::Right),
                _ => None,
            },
            Pipe::LUBend => match coming_from {
                Direction::Up => Some(Direction::Left),
                Direction::Left => Some(Direction::Up),
                _ => None,
            },
            Pipe::LDBend => match coming_from {
                Direction::Down => Some(Direction::Left),
                Direction::Left => Some(Direction::Down),
                _ => None,
            },
            Pipe::RUBend => match coming_from {
                Direction::Up => Some(Direction::Right),
                Direction::Right => Some(Direction::Up),
                _ => None,
            },
            Pipe::RDBend => match coming_from {
                Direction::Right => Some(Direction::Down),
                Direction::Down => Some(Direction::Right),
                _ => None,
            },
        }
    }

    fn can_move_from(&self, from_dir: &Direction) -> bool {
        return match self {
            Pipe::Vertical => [Direction::Up, Direction::Down],
            Pipe::Horizontal => [Direction::Left, Direction::Right],
            Pipe::RUBend => [Direction::Right, Direction::Up],
            Pipe::RDBend => [Direction::Right, Direction::Down],
            Pipe::LUBend => [Direction::Left, Direction::Up],
            Pipe::LDBend => [Direction::Left, Direction::Down],
            _ => return false,
        }.contains(&from_dir);
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn inverse(&self) -> Direction {
        return match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Right => Direction::Left,
            Direction::Left => Direction::Right,
        };
    }

    fn translate(&self, x: u32, y: u32) -> (u32, u32) {
        match self {
            Direction::Up => (x, y - 1),
            Direction::Right => (x + 1, y),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
        }
    }
}

struct PipeMap {
    rows: Vec<Vec<Pipe>>,
}

impl PipeMap {
    fn get_width(&self) -> u32 {
        return self.rows[0].len() as u32;
    }

    fn get_height(&self) -> u32 {
        return self.rows.len() as u32;
    }

    fn get_start(&self) -> (u32, u32) {
        for (y, row) in self.rows.iter().enumerate() {
            if let Some(col) = row.iter().position(|p| *p == Pipe::Start) {
                return (col as u32, y as u32);
            }
        }

        panic!("Failed to find starting position");
    }

    fn at(&self, x: u32, y: u32) -> Pipe {
        return self.rows[y as usize][x as usize];
    }

    fn get_next_direction(&self, x: u32, y: u32, last_dir: &Direction) -> Direction {
        return self.at(x, y).move_through(&last_dir).unwrap();
    }

    fn connects(&self, x1: u32, y1: u32, x2: u32, y2: u32) -> bool {
        if x1 >= self.get_width() || x2 >= self.get_width()
                || y1 >= self.get_height() || y2 >= self.get_height() {
            return false;
        }

        let pipe_a = self.at(x1, y1);
        let pipe_b = self.at(x2, y2);

        assert!(x1 != x2 || y1 != y2);

        if x1 != x2 {
            let (left, right) = if x2 > x1 { (pipe_a, pipe_b) } else { (pipe_b, pipe_a) };
            return (left == Pipe::Start || right == Pipe::Start)
                    || ([Pipe::Horizontal, Pipe::RUBend, Pipe::RDBend].contains(&left)
                    && [Pipe::Horizontal, Pipe::LUBend, Pipe::LDBend].contains(&right));
        } else {
            let (top, bottom) = if y2 > y1 { (pipe_a, pipe_b) } else { (pipe_b, pipe_a) };
            return (top == Pipe::Start || bottom == Pipe::Start)
                    || ([Pipe::Vertical, Pipe::LDBend, Pipe::RDBend].contains(&top)
                    && [Pipe::Vertical, Pipe::LUBend, Pipe::RUBend].contains(&bottom));
        }
    }
}

fn parse_map() -> PipeMap {
    return PipeMap { rows: read_input_file().iter().map(|line| line.chars().map(|c| {
        match c {
            'S' => Pipe::Start,
            '|' => Pipe::Vertical,
            '-' => Pipe::Horizontal,
            'J' => Pipe::LUBend,
            '7' => Pipe::LDBend,
            'L' => Pipe::RUBend,
            'F' => Pipe::RDBend,
            _ => Pipe::None,
        }
    }).collect()).collect() };
}

fn solve() -> (u32, u32) {
    let map = parse_map();

    let mut loop_tiles = HashSet::<(u32, u32)>::new();

    let (x0, y0) = map.get_start();

    loop_tiles.insert((x0, y0));

    let mut first_dir: Direction = Direction::Up;
    for dir in [Direction::Down, Direction::Up, Direction::Right, Direction::Left] {
        let (x1, y1) = dir.translate(x0, y0);
        if map.at(x1, y1).can_move_from(&dir.inverse()) {
            first_dir = dir;
            break;
        }
    }

    let mut cur_pos = first_dir.translate(x0, y0);
    loop_tiles.insert(cur_pos);
    let mut cur_pipe = map.at(cur_pos.0, cur_pos.1);
    let mut cur_dir = first_dir;
    let mut steps = 0;
    while cur_pipe != Pipe::Start {
        cur_dir = map.get_next_direction(cur_pos.0, cur_pos.1, &cur_dir);
        cur_pos = cur_dir.translate(cur_pos.0, cur_pos.1);
        loop_tiles.insert(cur_pos);
        cur_pipe = map.at(cur_pos.0, cur_pos.1);

        steps += 1;
        if steps > 100000 {
            println!("Too many steps");
            break;
        }
    }

    let furthest_dist = (steps + 1) / 2;

    // modified flood-fill algorithm to find all tiles _not_ enclosed by the loop

    // tiles filled by our flood
    let mut filled_tiles = HashSet::<(u32, u32)>::new();
    // the flood-fill actually operates on the spaces _between_ tiles,
    // so we track them separately here. 1 = the space between tiles 0 and 1.
    let mut visited_spaces = HashSet::<(u32, u32)>::new();

    let mut queue = VecDeque::<(u32, u32)>::new();
    queue.push_back((0, 0));
    while !queue.is_empty() {
        let (x, y) = queue.pop_front().unwrap();
        let mut check_dirs = Vec::<Direction>::new();
        if x > 0 {
            check_dirs.push(Direction::Left);
        }
        if x < map.get_width() {
            check_dirs.push(Direction::Right);
        }
        if y > 0 {
            check_dirs.push(Direction::Up);
        }
        if y < map.get_height() {
            check_dirs.push(Direction::Down);
        }

        for dir in check_dirs {
            let neighbor_space = match dir {
                Direction::Up => (x, y - 1),
                Direction::Right => (x + 1, y),
                Direction::Down => (x, y + 1),
                Direction::Left => (x - 1, y),
            };

            if visited_spaces.contains(&neighbor_space) {
                continue;
            }

            let p1 = match dir {
                Direction::Up => if x > 0 && y > 0 { Some((x - 1, y - 1)) } else { None },
                Direction::Right => if x < map.get_width() && y > 0 { Some((x, y - 1)) } else { None },
                Direction::Down => if x < map.get_width() && y < map.get_height() { Some((x, y)) } else { None },
                Direction::Left => if x > 0 && y < map.get_height() { Some((x - 1, y)) } else { None },
            };
            let p2 = match dir {
                Direction::Up => if x < map.get_width() && y > 0 { Some((x, y - 1)) } else { None },
                Direction::Right => if x < map.get_width() && y < map.get_height() { Some((x, y)) } else { None },
                Direction::Down => if x > 0 && y < map.get_height() { Some((x - 1, y)) } else { None },
                Direction::Left => if x > 0 && y > 0 { Some((x - 1, y - 1)) } else { None },
            };

            let blocked = p1.is_some()
                    && p2.is_some()
                    && (loop_tiles.contains(&p1.unwrap())
                            && loop_tiles.contains(&p2.unwrap())
                            && map.connects(p1.unwrap().0, p1.unwrap().1, p2.unwrap().0, p2.unwrap().1));

            if !blocked {

                queue.push_back(neighbor_space);

                visited_spaces.insert(neighbor_space);

                if p1.is_some() && !loop_tiles.contains(&p1.unwrap())
                        && p1.unwrap().0 < map.get_width()
                        && p1.unwrap().1 < map.get_height() {
                    filled_tiles.insert(p1.unwrap());
                }

                if p2.is_some() && !loop_tiles.contains(&p2.unwrap())
                        && p2.unwrap().0 < map.get_width()
                        && p2.unwrap().1 < map.get_height() {
                    filled_tiles.insert(p2.unwrap());
                }
            } else if p1.is_some() && p2.is_some() {
            }
        }
    }

    // add the tiles occupied by loop pipes and subtract the "border" we created
    let non_enclosed_area = filled_tiles.len() as u32 + loop_tiles.len() as u32;
    let enclosed_area = map.get_width() * map.get_height() - non_enclosed_area;

    return (furthest_dist, enclosed_area);
}
