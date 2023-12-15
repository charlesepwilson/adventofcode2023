use crate::utils::Solves;

pub struct Solution;

impl Solves for Solution {
    const DAY: u32 = 13;
    type ParsedInput = Vec<Vec<Vec<u8>>>;
    type Output = usize;

    fn parse_input(dir: &str) -> Self::ParsedInput {
        let lines: Vec<String> = Self::read_file(dir).collect();
        let sections = lines.split(|x| x.is_empty());
        let mut result = Vec::new();
        for section in sections {
            let block = section.iter().map(|x| x.as_bytes().to_vec()).collect();
            result.push(block);
        }
        result
    }

    fn part1(dir: &str) -> Self::Output {
        let input = Self::parse_input(dir);
        let mut tot_h = 0;
        let mut tot_v = 0;
        for block in input {
            let horizontal = find_horizontal_reflection_line(&block).unwrap_or(0);
            tot_h += horizontal;
            let vertical = find_vertical_reflection_line(&block).unwrap_or(0);
            tot_v += vertical;
        }
        tot_v + (100 * tot_h)
    }

    fn part2(dir: &str) -> Self::Output {
        let input = Self::parse_input(dir);
        0
    }
}


fn test_horizontal_reflection_line(block: &Vec<Vec<u8>>, at_index: usize) -> bool {
    for i in 1..=block.len() {
        if let Some(upper_index) = at_index.checked_sub(i){
            if let Some(lower_line) = block.get(at_index + i - 1) {
                let upper_line = &block[upper_index];
                if *upper_line != *lower_line {return false;}
            }
            else { true; }
        }
        else { true; }
    }
    true
}

fn find_horizontal_reflection_line(block: &Vec<Vec<u8>>) -> Option<usize> {
    for i in 1..=(block.len()-1) {
        if test_horizontal_reflection_line(block, i) {return Some(i);}
    }
    None
}

fn transpose(block: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let num_rows = block.len();
    let num_cols = block[0].len();
    let mut result = vec![vec![0u8;num_rows];num_cols];
    for (i, row) in block.iter().enumerate() {
        for (j, item) in row.iter().enumerate() {
            result[j][i] = *item;
        }
    }
    result
}

fn find_vertical_reflection_line(block: &Vec<Vec<u8>>) -> Option<usize> {
    let t = transpose(block);
    find_horizontal_reflection_line(&t)
}


// ideas for p2
// each block only has a certain sub-block of it that is relevant to reflection,
// where one or more rows or columns (but not both) have been removed from exactly one side of the block
// A valid reflection block MUST have an even number of #s and .s
// Swapping a . to a # or vice versa changes the parity of both
// So we need to find a sub-block that has an odd number of both #s and .s
// We also know that a sub block has an even number of rows or columns (along the reflected axis),
// so when checking for sub-blocks we can remove 2 rows at a time
//
// Once a potential sub block is found, is there a better way than just iterating
// through each potential reflect line and returning false if there's more than 1 difference?

const HASH: u8 = b'#';
const DOT: u8 = b'.';

fn is_valid_subblock(subblock: &[Vec<u8>]) -> bool {
    let num_hashes: usize = subblock.iter().map(|row| row.iter().filter(|&y| *y == HASH).count()).sum();
    let num_dots: usize = subblock.iter().map(|row| row.iter().filter(|&y| *y == DOT).count()).sum();
    ((num_hashes % 2) == 0) && ((num_dots % 2) == 0)
}
