use std::cmp::min;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::iter::Iterator;

#[derive(Clone, Copy)]
struct MappingRange {
    src_start: u32,
    src_end: u32,
    dst_start: u32,
    dst_end: u32,
    offset: i64,
}

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
            .collect();
}

fn parse_ranges<'a, I: Iterator<Item = &'a String>>(lines: &mut I) -> Vec<MappingRange> {
    let mut ranges = Vec::<MappingRange>::new();

    _ = lines.next(); // skip map heading
    for line in lines.take_while(|s| !s.is_empty()) {
        let mut spl = line.split_ascii_whitespace();
        let dst_start = str::parse::<u32>(spl.next().unwrap()).unwrap();
        let src_start = str::parse::<u32>(spl.next().unwrap()).unwrap();
        let len = str::parse::<u32>(spl.next().unwrap()).unwrap();

        // one of the mappings evaluates to exactly 2^32 - 1
        let src_end = (src_start as u64 + len as u64 - 1) as u32;
        let dst_end = (dst_start as u64 + len as u64 - 1) as u32;
        let offset: i64 = dst_start as i64 - src_start as i64;

        ranges.push(MappingRange {
            src_start,
            src_end,
            dst_start,
            dst_end,
            offset,
        });
    }
    _ = lines.next(); // skip blank line

    ranges.sort_by_key(|r| r.src_start);

    let mut imp_ranges = Vec::<MappingRange>::new();
    let first_range_start = ranges.first().unwrap().src_start;
    let last_range_end = ranges.last().unwrap().src_end;
    if first_range_start != 0 {
        imp_ranges.push(MappingRange {
            src_start: 0,
            src_end: first_range_start - 1,
            dst_start: 0,
            dst_end: first_range_start - 1,
            offset: 0,
        });
    }
    if last_range_end != u32::MAX {
        imp_ranges.push(MappingRange {
            src_start: last_range_end + 1,
            src_end: u32::MAX,
            dst_start: last_range_end + 1,
            dst_end: u32::MAX,
            offset: 0,
        });
    }

    for i in 1..ranges.len() {
        let range_a = &ranges[i - 1];
        let range_b = &ranges[i];
        if range_a.src_end + 1 != range_b.src_start {
            imp_ranges.push(MappingRange {
                src_start: range_a.src_end + 1,
                src_end: range_b.src_start - 1,
                dst_start: range_a.src_end + 1,
                dst_end: range_b.src_end - 1,
                offset: 0,
            });
        }
    }

    ranges.append(&mut imp_ranges);
    ranges.sort_by_key(|r| r.src_start);

    return ranges;
}

fn map_value(ranges: &Vec<MappingRange>, val: u32) -> u32 {
    for range in ranges {
        if range.src_end >= val && range.src_start <= val {
            return (val as i64 + range.offset) as u32;
        }
    }
    return val;
}

fn solve_a() -> u32 {
    let lines = read_input_file();

    let initial_seeds: Vec<u32> = lines[0][7..].split(" ").map(|s| str::parse::<u32>(s).unwrap()).collect();

    let mut it = lines.iter();
    // skip first two lines
    for _ in 0..2 {
        _ = it.next();
    }
    let seed_soil_map = parse_ranges(&mut it);
    let soil_fert_map = parse_ranges(&mut it);
    let fert_water_map = parse_ranges(&mut it);
    let water_light_map = parse_ranges(&mut it);
    let light_temp_map = parse_ranges(&mut it);
    let temp_hum_map = parse_ranges(&mut it);
    let hum_loc_map = parse_ranges(&mut it);

    let mut lowest_loc = u32::MAX;

    for seed in initial_seeds {
        let soil = map_value(&seed_soil_map, seed);
        let fert = map_value(&soil_fert_map, soil);
        let water = map_value(&fert_water_map, fert);
        let light = map_value(&water_light_map, water);
        let temp = map_value(&light_temp_map, light);
        let hum = map_value(&temp_hum_map, temp);
        let loc = map_value(&hum_loc_map, hum);

        if loc < lowest_loc {
            lowest_loc = loc;
        }
    }

    return lowest_loc;
}

fn compute_subranges(start_end: &Vec<(u32, u32)>, mapping_sets: &[&Vec<MappingRange>]) -> Vec<MappingRange> {
    let mut subranges = Vec::<MappingRange>::new();
    let mappings = mapping_sets[0];
    for (start, end) in start_end {
        let mut section_start: u64 = (*start).into();

        while section_start <= *end as u64 {
            for range in mappings {
                if section_start as u32 >= range.src_start && section_start as u32 <= range.src_end {
                    let section_end = min(end, &range.src_end);
                    subranges.push(MappingRange {
                        src_start: section_start as u32,
                        src_end: *section_end,
                        dst_start: (section_start as i64 + range.offset) as u32,
                        dst_end: (*section_end as i64 + range.offset) as u32,
                        offset: range.offset,
                    });
                    section_start = *section_end as u64 + 1;
                    break;
                }
            }
        }
    }

    if mapping_sets.len() > 1 {
        return compute_subranges(&subranges.iter().map(|r| (r.dst_start, r.dst_end)).collect(), &mapping_sets[1..]);
    } else {
        return subranges;
    }
}

fn solve_b() -> u32 {
    let lines = read_input_file();

    let initial_seeds: Vec<u32> = lines[0][7..].split(" ").map(|s| str::parse::<u32>(s).unwrap()).collect();
    let mut seed_ranges = Vec::<(u32, u32)>::new();
    for i in 0..initial_seeds.len() / 2 {
        seed_ranges.push((initial_seeds[i * 2], initial_seeds[i * 2 + 1]));
    }

    let mut it = lines.iter();
    // skip first two lines
    for _ in 0..2 {
        _ = it.next();
    }
    let seed_soil_map = parse_ranges(&mut it);
    let soil_fert_map = parse_ranges(&mut it);
    let fert_water_map = parse_ranges(&mut it);
    let water_light_map = parse_ranges(&mut it);
    let light_temp_map = parse_ranges(&mut it);
    let temp_hum_map = parse_ranges(&mut it);
    let hum_loc_map = parse_ranges(&mut it);

    let seeds_start_end: Vec<(u32, u32)> = seed_ranges.iter().map(|(start, len)| (*start, start + len - 1)).collect();

    let mapping_sets = vec![
        &seed_soil_map,
        &soil_fert_map,
        &fert_water_map,
        &water_light_map,
        &light_temp_map,
        &temp_hum_map,
        &hum_loc_map,
    ];

    let mut subranges = compute_subranges(&seeds_start_end, mapping_sets.as_slice());
    //let mut subranges = compute_subranges(&initial_seeds.iter().map(|s| (*s, *s)).collect(), mapping_sets.as_slice());

    subranges.sort_by_key(|r| r.src_start);
    let mut min_val = u32::MAX;
    for range in &subranges {
        let loc = (range.src_start as i64 + range.offset) as u32;
        if loc < min_val {
            min_val = loc;
        }
    }

    return min_val;
}
