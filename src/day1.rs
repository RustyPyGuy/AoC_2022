// Advent of Code Day TODAY
//
// Some imports not needed every time.
#![allow(dead_code)]
use aoc_22::*;
use std::io::Error; //import custom lib.rs module

const DAY: u8 = 1;
const TEST1_EXPECTED_OUTPUT: &str = "24000";
const TEST2_EXPECTED_OUTPUT: &str = "45000";

pub fn day_1_challenge_1(config: &Config) -> Result<i128, Error> {
    let puzzle_data = open_puzzle_file_to_buf(config);
    let mut vec_lines = read_buf_into_vec_lines(puzzle_data.unwrap()).unwrap();
    let mut vec_elf_totals: Vec<i32> = Vec::new();
    let mut current_elf_total = 0;
    vec_lines.push(0); // to handle last line.
    for line in vec_lines.into_iter() {
        if line == 0 {
            vec_elf_totals.push(current_elf_total);
            current_elf_total = 0;
        } else {
            current_elf_total += line;
        }
    }
    let mut max_elf = 0;
    for elf in vec_elf_totals.into_iter() {
        if elf > max_elf {
            max_elf = elf
        }
    }
    Ok(max_elf as i128)
}

pub fn day_1_challenge_2(config: &Config) -> Result<i128, Error> {
    let puzzle_data = open_puzzle_file_to_buf(config);
    let mut vec_lines = read_buf_into_vec_lines(puzzle_data.unwrap()).unwrap();
    let mut vec_elf_totals: Vec<i32> = Vec::new();
    vec_lines.push(0); // to handle last line.
    let mut current_elf_total = 0;
    for line in vec_lines.into_iter() {
        if line == 0 {
            vec_elf_totals.push(current_elf_total);
            current_elf_total = 0;
        } else {
            current_elf_total += line;
        }
    }
    let mut max_elf = 0;
    let mut max_elf2 = 0;
    let mut max_elf3 = 0;
    for elf in vec_elf_totals.into_iter() {
        if elf > max_elf3 {
            if elf > max_elf2 {
                if elf > max_elf {
                    max_elf3 = max_elf2;
                    max_elf2 = max_elf;
                    max_elf = elf;
                } else {
                    max_elf3 = max_elf2;
                    max_elf2 = elf;
                }
            } else {
                max_elf3 = elf;
            }
        } else {
            continue;
        }
    }
    dbg!(max_elf);
    dbg!(max_elf2);
    dbg!(max_elf3);
    let total_elf = max_elf + max_elf2 + max_elf3;
    Ok(total_elf as i128)
}

pub fn test_config_d0() -> Config {
    let test_config: Config = Config {
        challenge: 1,
        filename: "input/test1.txt".to_string(),
    };
    test_config
}

pub fn test_config_d1() -> Config {
    let test_config: Config = Config {
        challenge: 2,
        filename: "input/test1.txt".to_string(),
    };
    test_config
}
#[cfg(test)]
mod test {
    use super::*;
    test_expected_computed!(1, 1, test_config_d1());
    test_expected_computed!(1, 2, test_config_d1());
}
