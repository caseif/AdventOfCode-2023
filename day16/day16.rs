use std::cmp::max;
use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io;
use std::io::BufRead;
use Tile::Empty;

fn main() {
    let part_a = solve_a();
    println!("Part A: {}", part_a);
    let part_b = solve_b();
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

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Empty,
    Vertical,
    Horizontal,
    ForwardSlash,
    BackSlash,
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Beam {
    x: i32,
    y: i32,
    dir: Direction,
}

struct Grid {
    tiles: Vec<Vec<Tile>>,
}

impl Grid {
    fn width(&self) -> u32 {
        return self.tiles[0].len() as u32;
    }

    fn height(&self) -> u32 {
        return self.tiles.len() as u32;
    }

    fn at(&self, x: u32, y: u32) -> Tile {
        return self.tiles[y as usize][x as usize];
    }

    fn step_beam(&self, beam: Beam) -> Vec<Beam> {
        let (new_x, new_y) = match beam.dir {
            Direction::Up => (beam.x, beam.y - 1),
            Direction::Right => (beam.x + 1, beam.y),
            Direction::Down => (beam.x, beam.y + 1),
            Direction::Left => (beam.x - 1, beam.y),
        };

        if new_x < 0 || new_x >= self.tiles[0].len() as i32
                || new_y < 0 || new_y >= self.tiles.len() as i32 {
            return vec![];
        }

        let new_tile = self.at(new_x as u32, new_y as u32);

        return match new_tile {
            Empty => vec![Beam { x: new_x, y: new_y, dir: beam.dir }],
            Tile::Vertical => match beam.dir {
                Direction::Up | Direction::Down => vec![Beam { x: new_x, y: new_y, dir: beam.dir}],
                Direction::Right | Direction::Left => vec![
                    Beam { x: new_x, y: new_y, dir: Direction::Up },
                    Beam { x: new_x, y: new_y, dir: Direction::Down },
                ],
            },
            Tile::Horizontal => match beam.dir {
                Direction::Right | Direction::Left => vec![Beam { x: new_x, y: new_y, dir: beam.dir}],
                Direction::Up | Direction::Down => vec![
                    Beam { x: new_x, y: new_y, dir: Direction::Right },
                    Beam { x: new_x, y: new_y, dir: Direction::Left },
                ],
            },
            Tile::ForwardSlash => vec![Beam { x: new_x, y: new_y, dir: match beam.dir {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Up,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Down,
            } }],
            Tile::BackSlash => vec![Beam { x: new_x, y: new_y, dir: match beam.dir {
                Direction::Up => Direction::Left,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Up,
            } }],
        };
    }
}

fn read_grid() -> Grid {
    let mut rows = Vec::<Vec<Tile>>::new();
    for line in read_input_file() {
        rows.push(line.chars().map(|c| match c {
            '.' => Tile::Empty,
            '|' => Tile::Vertical,
            '-' => Tile::Horizontal,
            '/' => Tile::ForwardSlash,
            '\\' => Tile::BackSlash,
            _ => panic!(),
        }).collect());
    }
    return Grid { tiles: rows };
}

fn get_energized_tiles(grid: &Grid, start_x: i32, start_y: i32, start_dir: Direction) -> u32 {
    let mut ener_tiles = HashSet::<(u32, u32)>::new();
    let mut seen_beams = HashSet::<Beam>::new();
    let mut beams = VecDeque::<Beam>::new();
    beams.push_back(Beam { x: start_x, y: start_y, dir: start_dir });
    while !beams.is_empty() {
        let cur_beam = beams.pop_back().unwrap();
        if seen_beams.contains(&cur_beam) {
            continue;
        }

        if cur_beam.x >= 0 && cur_beam.y >= 0 {
            seen_beams.insert(cur_beam);
            ener_tiles.insert((cur_beam.x as u32, cur_beam.y as u32));
        }
        for new_beam in grid.step_beam(cur_beam) {
            beams.push_back(new_beam);
        }
    }

    return ener_tiles.len() as u32;
}

fn solve_a() -> u32 {
    let grid = read_grid();
    return get_energized_tiles(&grid, -1, 0, Direction::Right);
}

fn solve_b() -> u32 {
    let grid = read_grid();
    let mut highest = 0u32;
    for x in 0..grid.width() {
        highest = max(highest, get_energized_tiles(&grid, x as i32, -1, Direction::Down));
        highest = max(highest, get_energized_tiles(&grid, x as i32, grid.height() as i32, Direction::Up));
    }
    for y in 0..grid.height() {
        highest = max(highest, get_energized_tiles(&grid, -1, y as i32, Direction::Right));
        highest = max(highest, get_energized_tiles(&grid, grid.width() as i32, y as i32, Direction::Left));
    }
    return highest;
}
