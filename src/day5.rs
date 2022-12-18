// Advent of Code Day FIVE - The Fifth day of Christmas!
//
// Some imports not needed every time.
#![allow(dead_code)]
use aoc_22::*;
// use std::io::Error; //import custom lib.rs module
use std::io::{BufRead, BufReader, Error, Read, Seek, SeekFrom};

const DAY: u8 = 5;
const TEST1_EXPECTED_OUTPUT: &str = "CMZ";
const TEST2_EXPECTED_OUTPUT: &str = "MCD";

// Column, left to right, stack bottom to top of Characters
// indexing will be [column from left][row from bottom]
pub type CrateStack = Vec<Vec<char>>;
pub type CommandList = Vec<(usize, usize, usize)>;

/// Get the dimensions of the first portion of data
pub fn read_buf_calculate_grid<R: Read>(io: &mut R) -> (usize, usize) {
    let mut br = BufReader::new(io);
    let mut rows = 0;
    let mut col_stacks: usize = 0;
    for (index, line) in br.by_ref().lines().enumerate() {
        if line.as_ref().unwrap().starts_with(" 1") {
            rows = index;
            let values: Vec<usize> = line
                .unwrap_or_default()
                .split_whitespace()
                .filter_map(|x| x.parse::<usize>().ok())
                .collect();
            col_stacks = *values.iter().max().unwrap();
        }
    }
    (rows, col_stacks)
}
/// Read the data into fixed columns
pub fn read_buf_into_defined_columns2<R: Read>(
    io: &mut R,
    rows: usize,
    columns: usize,
) -> Result<CrateStack, std::io::Error> {
    let br = BufReader::new(io);
    let mut crate_stack: CrateStack = Vec::new();
    for _ in 0..columns {
        crate_stack.push(Vec::with_capacity(78)); // a macro could do this.
    }
    for (row, line) in br.lines().enumerate() {
        if row >= rows {
            break;
        }
        let line_str = line.unwrap().clone();
        let line_str2 = line_str.char_indices();
        for (index, char_) in line_str2 {
            if char_.is_alphabetic() {
                let insert_index = (index - 1) / 4;
                crate_stack[insert_index].push(char_);
            }
        }
    }

    for index in 0..crate_stack.len() {
        crate_stack[index].reverse(); // reverse the stack so top element is last
    }
    Ok(crate_stack)
}

/// Read the moving commands line by line
pub fn read_buf_into_command_list<R: Read>(io: &mut R) -> Result<CommandList, std::io::Error> {
    let mut command_list: Vec<(usize, usize, usize)> = Vec::new();
    let br = BufReader::new(io);
    for line in br.lines() {
        if !line.as_ref().unwrap().starts_with("move") {
            continue;
        }
        let values2: Vec<usize> = line
            .unwrap_or_default()
            .split_whitespace()
            .filter_map(|x| x.parse::<usize>().ok())
            .collect();
        // I like tuple indexing better.
        command_list.push((values2[0], values2[1], values2[2]));
    }
    Ok(command_list)
}

pub fn day_5_challenge_1(config: &Config) -> Result<String, Error> {
    let mut buf = open_puzzle_file_to_buf(config).unwrap();
    let (rows, columns): (usize, usize) = read_buf_calculate_grid(&mut buf);
    buf.seek(SeekFrom::Start(0)).expect("file buffer error");
    let mut crate_stack = read_buf_into_defined_columns2(&mut buf, rows, columns).unwrap();
    buf.seek(SeekFrom::Start(0)).expect("file buffer error");
    let command_list = read_buf_into_command_list(&mut buf).unwrap();
    #[allow(unused_assignments)]
    let mut stacking_vec: Vec<char> = Vec::with_capacity(72);
    for command in command_list.into_iter() {
        let from_i = command.1 - 1;
        let to_i = command.2 - 1;
        let vert_split_i: usize = crate_stack[from_i].len() - command.0;
        stacking_vec = crate_stack[from_i].split_off(vert_split_i);
        stacking_vec.reverse();
        crate_stack[to_i].append(&mut stacking_vec);
    }
    let mut output_string = String::new();
    for mut crate_ in crate_stack.into_iter() {
        let output_char: char = crate_.pop().unwrap_or('_');
        if output_char != '_' {
            output_string.push(output_char);
        }
    }
    println!("output: {}", output_string);
    Ok(output_string)
}

pub fn day_5_challenge_2(config: &Config) -> Result<String, Error> {
    let mut buf = open_puzzle_file_to_buf(config).unwrap();
    let (rows, columns): (usize, usize) = read_buf_calculate_grid(&mut buf);
    buf.seek(SeekFrom::Start(0)).expect("file buffer error");
    // let mut crate_stack = read_buf_into_defined_columns(&mut buf, rows, columns, 4).unwrap();
    let mut crate_stack = read_buf_into_defined_columns2(&mut buf, rows, columns).unwrap();
    buf.seek(SeekFrom::Start(0)).expect("file buffer error");
    let command_list = read_buf_into_command_list(&mut buf).unwrap();
    #[allow(unused_assignments)]
    let mut stacking_vec: Vec<char> = Vec::with_capacity(72);
    for command in command_list.into_iter() {
        let from_i = command.1 - 1;
        let to_i = command.2 - 1;
        let vert_split_i: usize = crate_stack[from_i].len() - command.0;
        stacking_vec = crate_stack[from_i].split_off(vert_split_i);
        // stacking_vec.reverse();  // ONE CHANGED LINE!!
        crate_stack[to_i].append(&mut stacking_vec);
    }
    let mut output_string = String::new();
    for mut crate_ in crate_stack.into_iter() {
        let output_char: char = crate_.pop().unwrap_or('_');
        if output_char != '_' {
            output_string.push(output_char);
        }
    }
    println!("output: {}", output_string);
    Ok(output_string)
}

pub fn test_config_d5() -> Config {
    let test_config: Config = Config {
        challenge: 5,
        filename: "./input/test5.txt".to_string(),
    };
    test_config
}

#[cfg(test)]
mod test {
    use super::*;
    test_expected_computed_string!(5, 1, test_config_d5());
    test_expected_computed_string!(5, 2, test_config_d5());
}
