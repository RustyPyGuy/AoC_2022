// Advent of Code Day FOUR
//
// Some imports not needed every time.
#![allow(dead_code)]
use aoc_22::*;
use std::io::Error; //import custom lib.rs module

const DAY: u8 = 4;
const TEST1_EXPECTED_OUTPUT: &str = "2";
const TEST2_EXPECTED_OUTPUT: &str = "4";

/// function to parse the input into a tuple of four coordinates
/// first range in first 2 elements and second range in second elements
pub fn parse_to_coords(config: &Config) -> Vec<(usize, usize, usize, usize)> {
    let file_buffer = open_puzzle_file_to_buf(config).unwrap();
    let lines = read_buf_into_vec_strings(file_buffer).unwrap();
    let mut coord_vec: Vec<(usize, usize, usize, usize)> = Vec::new();
    for line in lines.into_iter() {
        let mut coords: (usize, usize, usize, usize) = (0, 0, 0, 0);
        let mut line_split = line.split(',');
        // first half of the split
        let mut numbers = line_split
            .next()
            .unwrap()
            .split('-')
            .map(|s| s.parse::<usize>().unwrap());
        coords.0 = numbers.next().unwrap();
        coords.1 = numbers.next().unwrap();
        // next half of the split
        let mut numbers = line_split
            .next()
            .unwrap()
            .split('-')
            .map(|s| s.parse::<usize>().unwrap());
        coords.2 = numbers.next().unwrap();
        coords.3 = numbers.next().unwrap();
        coord_vec.push(coords);
    }
    coord_vec
}

pub fn day_4_challenge_1(config: &Config) -> Result<i128, Error> {
    let mut count: i128 = 0;
    for coords in parse_to_coords(config).into_iter() {
        if coords.0 <= coords.2 && coords.1 >= coords.3 {
            // first set captures

            count += 1;
        } else if coords.2 <= coords.0 && coords.3 >= coords.1 {
            // second set caputures
            count += 1;
        }
    }
    Ok(count)
}

pub fn day_4_challenge_2(config: &Config) -> Result<i128, Error> {
    let mut count: i128 = 0;
    for coords in parse_to_coords(config).into_iter() {
        if coords.0 <= coords.2 && coords.1 >= coords.3 {
            // first set captures
            count += 1;
        } else if coords.2 <= coords.0 && coords.3 >= coords.1 {
            // second set caputures
            count += 1;
        } else if coords.1 >= coords.2 && coords.3 >= coords.1 {
            // second set min is within bounds of first set
            count += 1;
        } else if coords.3 >= coords.0 && coords.1 > coords.3 {
            // first set min is within bounds of second set
            count += 1;
        }
    }
    Ok(count)
}

pub fn test_config_d4() -> Config {
    let test_config: Config = Config {
        challenge: 4,
        filename: "./input/test4.txt".to_string(),
    };
    test_config
}

#[cfg(test)]
mod test {
    use super::*;
    test_expected_computed!(4, 1, test_config_d4());
    test_expected_computed!(4, 2, test_config_d4());
}
