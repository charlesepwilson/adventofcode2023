use std::collections::VecDeque;
use std::iter::zip;
use crate::utils::Solves;

pub struct Solution;
const OPERATIONAL: char = '.';
const DAMAGED: char = '#';
const UNKNOWN: char = '?';

impl Solves for Solution {
    const DAY: u32 = 12;
    type ParsedInput = Vec<(String, Vec<usize>)>;
    type Output = usize;

    fn parse_input(dir: &str) -> Self::ParsedInput {
        Self::read_file(dir).map(parse_line).collect()
    }

    fn part1(dir: &str) -> Self::Output {
        let input = Self::parse_input(dir);
        sum_ways(input)
    }

    fn part2(dir: &str) -> Self::Output {
        let input = Self::parse_input(dir);
        let unfolded = input;
        sum_ways(unfolded)
    }
}
fn parse_line(line: String) -> (String, Vec<usize>) {
    let (map, numbers_str) = line.split_once(" ").unwrap();
    let numbers: Vec<usize> = numbers_str.split(",").map(|x| x.parse().unwrap()).collect();
    (map.to_string(), numbers)
}


fn compute_unknown_arrangements(num_unknown: usize, numbers: Vec<usize>) -> usize {
    if num_unknown == 0 {return 1;}
    let num_blocks = numbers.len();
    let sum: usize = numbers.iter().sum();
    let space_required = (sum + num_blocks).saturating_sub(1);
    if space_required > num_unknown {return  0;}
    if space_required == num_unknown {return 1;}
    if num_blocks == 1 {
        return 1 + num_unknown - sum;
    }
    if num_blocks == 2 {
        return compute_unknown_arrangements2(num_unknown, numbers);
    }
    if num_blocks == 3 {
        return compute_unknown_arrangements3(num_unknown, numbers);
    }
    let mut total = 0;
    for i in 1..=(num_unknown - space_required ) {
        total += compute_unknown_arrangements(
            num_unknown - numbers[0] - i,
            numbers[1..].to_vec(),
        )
    }
    return total;

}

fn compute_unknown_arrangements2(num_unknown: usize, numbers: Vec<usize>,) -> usize {
    // if you just have a group of ?s, with 2 groups of #s inside,
    // this is the number of ways of arranging them
    let n = num_unknown - numbers.iter().sum::<usize>();
    (n * (n+1)) / 2
}

fn compute_unknown_arrangements3(num_unknown: usize, numbers: Vec<usize>) -> usize {
    // if you just have a group of ?s, with 3 groups of #s inside,
    // this is the number of ways of arranging them
    let n = num_unknown - numbers.iter().sum::<usize>();
    (n * (n+1) * (n-1)) / 6
}

fn count_ways(row: String, numbers: Vec<usize>) -> usize {
    let l = row.len();
    if l == 0 {return 1;}
    if row.chars().all(|x| x == UNKNOWN) {
        return compute_unknown_arrangements(l, numbers);
    }
    if row.chars().all(|x| (x == UNKNOWN) | (x == OPERATIONAL)) {
        let sections: Vec<_> = row.split(OPERATIONAL).map(|x| x.chars().count()).collect();
        if sections.len() == numbers.len() {
            return zip(sections, numbers).map(|(space, item)| compute_unknown_arrangements(space, vec![item])).sum()
        }
    }
    println!("Haven't dealt with this case yet {} {:?}", row, numbers);
    0
}

pub fn guarantee_hashes(size: usize, numbers: &Vec<usize>) -> Vec<char> {
    if size == 0 { return Vec::new();}
    let total_required_space: usize = numbers.iter().sum::<usize>() + numbers.len() - 1;
    let wiggle_room: usize = size.saturating_sub(total_required_space);
    let mut row = vec![OPERATIONAL;size];
    for (index, n) in numbers.iter().enumerate() {
        let num_hashes = n.saturating_sub(wiggle_room);
        let previous_nums = &numbers[0..index];
        let offset = previous_nums.iter().sum::<usize>() + previous_nums.iter().count();
        for i in (n-num_hashes)..*n {
            row[i + offset] = DAMAGED;
        }

    }
    row
}

fn fill_in_hashes(row: String, numbers: &Vec<usize>) -> Vec<char> {
    let total_size = row.chars().count();
    let guaranteed_hashes = guarantee_hashes(total_size, &numbers);
    let combined: String = zip(row.chars(), guaranteed_hashes).map(|(r, g)| if g == DAMAGED {g} else {r}).collect();
    let stripped = strip_operational(combined);
    stripped.chars().collect()
}

fn remove_guaranteed_blocks(mut row: String, numbers: Vec<usize>) -> (String, Vec<usize>) {
    row = ".".to_string() + row.as_str() + ".";
    let mut number_indices_to_remove = Vec::new();
    for (i, n) in numbers.iter().enumerate() {
        let pat_s = generate_guaranteed_block_pattern(*n);
        let pat = pat_s.as_str();
        if let Some(index) = row.find(pat) {
            if !row.chars().take(index).any(|x| x == UNKNOWN) {
                row = row.replacen(pat, ".", 1);
                number_indices_to_remove.push(i);
                continue;
            }
        }
        if let Some(index) = row.rfind(pat) {
            if !row.chars().skip(index).any(|x| x == UNKNOWN) {
                row = row.replacen(pat, ".", 1);
                number_indices_to_remove.push(i);
            }
        }
    }
    let mut new_numbers = Vec::new();
    for (i, element) in numbers.iter().enumerate() {
        if !number_indices_to_remove.contains(&i) {
            new_numbers.push(*element);
        }
    }
    (row, new_numbers)

}

fn n_damaged_pattern(n: usize) -> String {
    vec![DAMAGED;n].iter().collect()
}

fn pad_row(row: String) -> String {
    let mut s = OPERATIONAL.to_string();
    s += row.as_str();
    s.push(OPERATIONAL);
    s
}

fn generate_guaranteed_block_pattern(n: usize) -> String {
    pad_row(n_damaged_pattern(n))
}

fn strip_operational(row: String) -> String {
    let new_row = row.trim_matches(OPERATIONAL);
    new_row.to_string()
}

fn collapse_duplicate_operational(mut row: String) -> String {
    loop {
        let l = row.len();
        row = row.replace("..", ".");
        if l == row.len() {return row;}
    }
}

fn simplify(row: String) -> String {
    strip_operational(collapse_duplicate_operational(row))
}

fn fill_in_end_groups(row: String, numbers: &Vec<usize>) -> String {
    if numbers.len() == 0 {return row;}
    // assumes this is simplified, so the first character MUST be a ? or #
    let first = numbers[0];
    let mut row_vec: Vec<_> = row.chars().collect();
    if row_vec.len() == first {
        return vec![DAMAGED;first].iter().collect();
    }

    if row_vec[0] == DAMAGED {
        for i in 0..first {
            row_vec[i] = DAMAGED;
        }
        row_vec[first] = OPERATIONAL;
    }
    let threshold = first + 1;
    let substr: String = row_vec[0..threshold].iter().collect();
    if substr.contains(DAMAGED) {
        // then this MUST be the first group
        if *row_vec.get(threshold).unwrap_or(&';') == OPERATIONAL {
            for i in 0..first {row_vec[i] = DAMAGED;}
        }
        if substr.find(n_damaged_pattern(first).as_str()) != None {
            for i in 0..threshold {
                if row_vec[i] == UNKNOWN {row_vec[i] = OPERATIONAL;}
            }
        }
    }



    let last = *numbers.last().unwrap();
    let back_threshold = row_vec.len() - last - 1;

    if *row_vec.last().unwrap() == DAMAGED {
        for i in (back_threshold+1)..row_vec.len() {
            row_vec[i] = DAMAGED;
        }
        row_vec[back_threshold] = OPERATIONAL;
    }

    let substr: String = row_vec[back_threshold..].iter().collect();
    if substr.contains(DAMAGED) {
        // last group
        if *row_vec.get(back_threshold).unwrap_or(&';') == OPERATIONAL  {
            for i in (back_threshold+1)..row_vec.len() { row_vec[i] = DAMAGED;}
        }
        if substr.find(n_damaged_pattern(last).as_str()) != None {
            for i in back_threshold..row_vec.len() {
                if row_vec[i] == UNKNOWN {row_vec[i] = OPERATIONAL;}
            }
        }
    }
    row_vec.into_iter().collect()
}

fn remove_end_groups(row: String, numbers: Vec<usize>) -> (String, Vec<usize>) {
    if numbers.len() == 0 {return (row, numbers);}
    let mut num_deq: VecDeque<_> = numbers.iter().map(|x| *x).collect();
    let mut row_vec: Vec<_> = row.chars().collect();
    if row_vec[..numbers[0]].iter().all(|x| *x == DAMAGED) {
        row_vec = row_vec[numbers[0]..].to_vec();
        if row_vec.len() != 0 {row_vec[0] = OPERATIONAL;}

        num_deq.pop_front();
    }
    if num_deq.len() == 0 {return (row_vec.into_iter().collect(), num_deq.into_iter().collect());}
    let l = row_vec.len();
    let threshold = l - num_deq.iter().last().unwrap();
    if row_vec[threshold..].iter().all(|x| *x == DAMAGED) {
        row_vec = row_vec[..threshold].to_vec();
        let l = row_vec.len();
        if l > 0{
            let last_index = l - 1;
            row_vec[last_index] = OPERATIONAL;
        }

        num_deq.pop_back();
    }
    (row_vec.into_iter().collect(), num_deq.into_iter().collect())
}

fn try_assume_blank(row: String, numbers: &Vec<usize>) -> String {
    let possible_row = row.replace(UNKNOWN, OPERATIONAL.to_string().as_str());
    if is_valid(&possible_row, numbers) {
        return possible_row;
    }
    return row;
}

fn is_valid(row: &String, numbers: &Vec<usize>) -> bool {
    let whitespace = row.clone().replace(OPERATIONAL, " ");
    let groups = whitespace.split_whitespace();
    let counts: Vec<_> = groups.map(|x| x.len()).collect();
    if counts.len() != numbers.len() {return false;}
    zip(counts, numbers).all(|(l, r)| l == *r)
}

fn remove_dead_blanks(row: String, numbers: &Vec<usize>) -> String {
    dbg!(&row, numbers);
    if numbers.len() == 0 {return row;}
    let min_num = *numbers.iter().min().unwrap();
    let mut padded_row = pad_row(row);
    let row_vec: Vec<_> = padded_row.chars().collect();
    let unknown_sections = padded_row.match_indices(".?");
    let mut sections_to_replace = Vec::new();
    for (start, _) in unknown_sections {
        let mut i = start;
        let mut c = UNKNOWN;

        while c == UNKNOWN {
            i = i + 1;
            c = row_vec[i];
        }
        if (c == OPERATIONAL) & ((i - start) < min_num){
            sections_to_replace.push((start, i));
        }
    }
    for (start, end) in sections_to_replace.iter().rev() {
        padded_row.replace_range(start..=end, ".");
    }
    dbg!(&padded_row);
    padded_row
}


fn apply_all_simplification_strategies(mut row: String, mut numbers: Vec<usize>) -> (String, Vec<usize>) {
    row = fill_in_hashes(row, &numbers).into_iter().collect();
    (row, numbers) = remove_guaranteed_blocks(row, numbers);
    dbg!(&row, &numbers);
    row = simplify(row);
    row = fill_in_end_groups(row, &numbers);
    dbg!(&row, &numbers);

    row = simplify(row);
    (row, numbers) = remove_end_groups(row, numbers);
    dbg!(&row, &numbers);

    row = simplify(row);
    row = try_assume_blank(row, &numbers);
    dbg!(&row, &numbers);

    row = simplify(row);
    row = remove_dead_blanks(row, &numbers);
    dbg!(&row, &numbers);

    row = simplify(row);
    (row, numbers)
}

fn recursively_apply_all_simplification_strategies(mut row: String, mut numbers: Vec<usize>) -> (String, Vec<usize>) {
    loop {
        let previous_row = row.clone();
        (row, numbers) = apply_all_simplification_strategies(row, numbers);
        if row == previous_row {
            return (row, numbers);
        }
    }
}

fn sum_ways(input: Vec<(String, Vec<usize>)>) -> usize {
    let mut total = 0;
    for (m, n) in input {
        dbg!(&m, &n);
        let (row, numbers) = recursively_apply_all_simplification_strategies(m, n);
        total += count_ways(row, numbers);
    }
    total
}