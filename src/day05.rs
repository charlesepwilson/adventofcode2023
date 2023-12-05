use std::slice::Iter;
use crate::utils::read_lines;

pub fn part1() -> usize {
    let (seeds, maps) = parse_input();
    let locations: Vec<usize> = seeds.iter().map(|&x| find_location(x, &maps)).collect();
    *locations.iter().min().unwrap()
}
pub fn part2() -> usize {
    let r = 0;
    r
}


fn parse_input() -> (Vec<usize>, Vec<Vec<(usize, usize, usize)>>) {
    let mut lines = Vec::new();
    if let Ok(buf_lines) = read_lines("./input/day05.txt") {
        for line in buf_lines {
            if let Ok(ip) = line {
                lines.push(ip);
            }
        }
    }
    let sections = lines.split(|x| x.is_empty());
    let mut sec_iter = sections.into_iter();
    let seeds_str = sec_iter.next().unwrap().iter().next().unwrap();
    let seeds = parse_seeds(seeds_str);
    let mut maps = Vec::new();
    for section in sec_iter {
        let mut iter = section.iter();
        let _ = iter.next().unwrap();
        let map = parse_section(iter);
        maps.push(map);
    }
    (seeds, maps)
}

fn parse_seeds(seeds_str: &String) -> Vec<usize> {
    seeds_str.strip_prefix("seeds: ").unwrap().split_whitespace().map(|x| x.parse().unwrap()).collect()
}

fn parse_section(lines: Iter<String>) -> Vec<(usize, usize, usize)> {
    let mut maps = Vec::new();
    for line in lines {
        let parts: Vec<usize> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
        maps.push(( parts[0], parts[1], parts[2]));
    }
    maps
}

fn find_location(seed: usize, maps: &Vec<Vec<(usize, usize, usize)>>) -> usize {
    let mut value = seed;
    for map in maps {
        for (min_dest, min_source, range) in map {
            if (*min_source <= value) & (value < min_source + range) {
                value = (value + min_dest) - min_source;
                break;
            }
        }
    }
    value
}
