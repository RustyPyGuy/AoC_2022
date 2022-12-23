// Advent of Code Day SEVENTEEN
//
// Some imports not needed every time.
#![allow(dead_code)]
use aoc_22::*; //import custom lib.rs module
use std::io::Error; // use std::process;
                    // use grid::Grid;
                    // use itertools::{Itertools, MultiPeek};
                    // use std::collections::{HashMap, HashSet};

const DAY: u8 = 17;
const TEST1_EXPECTED_OUTPUT: &str = "0";
const TEST2_EXPECTED_OUTPUT: &str = "0";

#[derive(Debug, Copy, Clone)]
pub struct DataStruct {
    x: i32,
    y: i32,
    z: i32,
}

impl DataStruct {
    pub fn new() -> Result<DataStruct, &'static str> {
        let x = 0;
        let y = 0;
        let z = 0;
        Ok(DataStruct { x, y, z })
    }
    pub fn export(self) -> Result<DataStruct, &'static str> {
        // return Ok(Coordinates{self.x,self.y,self.z});
        println!("export method called returning struct {:?}", self);
        Ok(self)
    }
}

pub fn day_17_challenge_1(config: &Config) -> Result<i128, Error> {
    let buf = open_puzzle_file_to_buf(config).unwrap();
    let _vec_strings = read_buf_into_vec_strings(buf);
    let test_pass: i128 = 0;
    Ok(test_pass)
}

pub fn day_17_challenge_2(config: &Config) -> Result<i128, Error> {
    let buf = open_puzzle_file_to_buf(config).unwrap();
    let _vec_strings = read_buf_into_vec_strings(buf);
    let test_pass: i128 = 0;
    Ok(test_pass)
}

pub fn test_config_d17() -> Config {
    let test_config: Config = Config {
        challenge: 0,
        filename: "./input/test17.txt".to_string(),
    };
    test_config
}

#[cfg(test)]
mod test {
    use super::*;
    test_expected_computed!(17, 1, test_config_d17());
    test_expected_computed!(17, 2, test_config_d17());
}
