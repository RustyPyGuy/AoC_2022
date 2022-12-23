// Advent of Code Day TWELVE
//
// Some imports not needed every time.
#![allow(dead_code)]
use aoc_22::*;
use std::io::Error; //import custom lib.rs module

const DAY: u8 = 0;
const TEST1_EXPECTED_OUTPUT: &str = "0";
const TEST2_EXPECTED_OUTPUT: &str = "0";
use grid::Grid;

#[derive(Debug, Copy, Clone)]
pub struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    pub fn new() -> Coord {
        Coord { x: 0, y: 0 }
    }
    #[allow(unused_assignments)]
    pub fn set(mut self, x: usize, y: usize) {
        self = Coord { x, y };
    }
}

pub fn day_12_challenge_1(config: &Config) -> Result<i128, Error> {
    let buf = open_puzzle_file_to_buf(config).unwrap();
    let vec_strings = read_buf_into_vec_strings(buf).unwrap();
    let (elevation_map, start_coord, end_coord) = map_elevations(vec_strings);
    dbg!(&elevation_map, &start_coord, &end_coord);
    let test_fail: i128 = i128::MAX;
    Ok(test_fail)
}
fn map_elevations(vec_strings: Vec<String>) -> (Grid<usize>, Coord, Coord) {
    let start_coord: Coord = Coord::new();
    let end_coord: Coord = Coord::new();
    // count size
    let input_rows = vec_strings.len();
    // NOTE: need to check that the len of string does what I think.
    let input_cols = vec_strings[0].len();
    let mut elevation: Grid<usize> = Grid::new(input_rows, input_cols);
    let letter_lookup = generate_priority_lookup_map_d14();
    for row in vec_strings.iter().enumerate() {
        for letter in row.1.chars().enumerate() {
            if letter.1 == 'S' {
                // start_coord = Coord::set(row.0,letter.0);
                start_coord.set(row.0, letter.0);
                elevation[row.0][letter.0] = 0;
            } else if letter.1 == 'E' {
                end_coord.set(row.0, letter.0);
                elevation[row.0][letter.0] = 27;
            } else if letter.1.is_lowercase() {
                elevation[row.0][letter.0] = *letter_lookup.get(&letter.1).unwrap();
            } else {
                panic!("input error");
            }
        }
    }
    (elevation, start_coord, end_coord)
}

pub fn day_12_challenge_2(config: &Config) -> Result<i128, Error> {
    let buf = open_puzzle_file_to_buf(config).unwrap();
    let _vec_strings = read_buf_into_vec_strings(buf);
    let test_pass: i128 = 0;
    Ok(test_pass)
}

pub fn generate_priority_lookup_map_d14() -> std::collections::HashMap<char, usize> {
    let mut elevation_score: std::collections::HashMap<char, usize> =
        std::collections::HashMap::with_capacity(30);
    let mut score_index: usize = 1;
    for char_each in "abcdefghijklmnopqrstuvwxyz".chars() {
        elevation_score.insert(char_each, score_index);
        score_index += 1;
    }
    elevation_score
}

pub fn test_config_d12() -> Config {
    let test_config: Config = Config {
        challenge: 0,
        filename: "./input/test12.txt".to_string(),
    };
    test_config
}

#[cfg(test)]
mod test {
    use super::*;
    test_expected_computed!(12, 1, test_config_d12());
    test_expected_computed!(12, 2, test_config_d12());
}
