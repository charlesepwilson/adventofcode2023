use std::slice::Iter;
use crate::utils::read_lines;

pub fn part1() -> usize {
    let (seeds, layers) = parse_input();
    find_min_location(seeds, layers)
}
pub fn part2() -> usize {
    let (seeds, layers) = parse_input();
    let seed_pairs: Vec<(usize, usize)> = seeds.chunks_exact(2).map(|x| (x[0], x[1])).collect();
    let mut bands = Vec::new();
    for (start, range) in seed_pairs {
        bands.push((start, start + range - 1));
    }
    for layer in layers {
        let mut new_bands = Vec::new();
        for (band_min, band_max) in bands.iter() {
            new_bands.extend(layer.split_band(*band_min, *band_max));
        }
        let mut traversed_bands = Vec::new();
        for (band_min, band_max) in new_bands.iter() {
            traversed_bands.push(layer.traverse_band(*band_min, *band_max));
        }
        bands = traversed_bands
    }
    let mut locations = Vec::new();
    for (b, t) in bands {
        locations.push(b);
        locations.push(t);
    }
    *locations.iter().min().unwrap()
}

fn find_min_location<I>(seeds: I, layers: Vec<Layer>) -> usize
    where
        I: IntoIterator<Item = usize>,
{
    let mut locations = Vec::new();
    for seed in seeds {
        locations.push(find_location(seed, &layers));
    }
    *locations.iter().min().unwrap()
}


fn parse_input() -> (Vec<usize>, Vec<Layer>) {
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
    (seeds, layers_from_maps(maps))
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

fn layers_from_maps(maps: Vec<Vec<(usize, usize, usize)>>) -> Vec<Layer> {
    let mut layers = Vec::new();
    for map in maps {
        layers.push(Layer::from_ranges(map));
    }
    layers
}

fn find_location(seed: usize, layers: &Vec<Layer>) -> usize {
    let mut value = seed;
    for layer in layers {
        value = layer.traverse(value);
    }
    value
}

fn in_band(value: usize, band_min: usize, band_max: usize) -> bool {
    (value >= band_min) & (value <= band_max)
}

struct Layer {
    pipes: Vec<(usize, usize, isize)>,
}

impl Layer {
    fn from_ranges(mut ranges: Vec<(usize, usize, usize)>) -> Self {
        let mut pipes = Vec::new();
        // rethink each map as a series of "pipes"
        // each of which adds a modifier to values that fall within it's range
        ranges.sort_by_key(|(_, s, _)| *s);
        let mut last_pipe_end: usize = 0;
        for (min_dest, min_source, range) in  ranges {
            let pipe_start = min_source;
            let pipe_end = pipe_start + range - 1;
            let modifier = (min_dest as isize) - (min_source as isize);
            let dead_pipe_start = last_pipe_end + 1;
            // Add "dead pipe" to explicitly handle the case where the value isn't specified
            if (dead_pipe_start != pipe_start)  & (dead_pipe_start != 1) {
                pipes.push((dead_pipe_start, pipe_start - 1, 0));
            }
            pipes.push((pipe_start, pipe_end, modifier));
            last_pipe_end = pipe_end;
        }
        Self{pipes}
    }

    fn traverse(&self, value: usize) -> usize {
        for (pipe_start, pipe_end, modifier) in &self.pipes {
            if in_band(value, *pipe_start, *pipe_end) {
                return ((value as isize) + modifier) as usize;
            }
        }
        value
    }

    fn split_band(&self, band_min: usize, band_max: usize) -> Vec<(usize, usize)> {
        let mut new_bands: Vec<(usize, usize)> = Vec::new();
        for (start, end, _) in self.pipes.iter().cloned() {
            let start_in = in_band(start, band_min, band_max);
            let end_in = in_band(end, band_min, band_max);
            if (start <= band_min) & end_in {
                new_bands.push((band_min, end))
            }
            if start_in & end_in {
                new_bands.push((start, end));
            }
            if start_in & (end >= band_max) {
                new_bands.push((start, band_max));
            }
            if (start <= band_min) & (end >= band_max) {
                new_bands.push((band_min, band_max));
            }
        }
        new_bands
    }

    fn traverse_band(&self, band_min: usize, band_max: usize) -> (usize, usize) {
        (self.traverse(band_min), self.traverse(band_max))
    }
}
