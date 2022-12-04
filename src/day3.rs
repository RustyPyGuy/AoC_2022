// Advent of Code Day THREEE
//
// Some imports not needed every time.
#![allow(dead_code)]
use aoc_22::*;
use std::collections::HashMap;
use std::io::Error; //import custom lib.rs module

const DAY: u8 = 3;
const TEST1_EXPECTED_OUTPUT: &str = "157";
const TEST2_EXPECTED_OUTPUT: &str = "70";

pub fn generate_priority_lookup_map() -> std::collections::HashMap<char, u8> {
    let mut char_score: std::collections::HashMap<char, u8> = std::collections::HashMap::new();
    let mut score_index: u8 = 1;
    for char_each in "abcdefghijklmnopqrstuvwxyz".chars() {
        char_score.insert(char_each, score_index);
        score_index += 1;
    }
    for char_each in "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars() {
        char_score.insert(char_each, score_index);
        score_index += 1;
    }
    char_score
}

pub fn day_3_challenge_1(config: &Config) -> Result<i128, Error> {
    let file_buffer = open_puzzle_file_to_buf(config).unwrap();
    let lines = read_buf_into_vec_strings(file_buffer).unwrap();
    let mut priority_count: i128 = 0;
    let priority_lookup_map = generate_priority_lookup_map();
    let mut priority_verified = String::new();
    for line in lines.into_iter() {
        let len = line.len(); // assuming no multibyte characters
        let (half1, half2): (&str, &str) = line.as_str().split_at(len / 2);
        let mut matched: bool = false;
        for item1 in half1.chars() {
            if matched == true {
                break;
            }
            for item2 in half2.chars() {
                if matched == true {
                    // I learned something about iterators here.  The break in the parent loop
                    // didn't reset this iterator with a new expression half2.chars.  Hence the
                    // break
                    break;
                }
                if item1 == item2 {
                    priority_verified.push(item1);
                    priority_count += *priority_lookup_map.get(&item1).unwrap() as i128;
                    matched = true;
                    // dbg!(priority_count);
                }
            }
        }
    }
    Ok(priority_count)
}

pub fn day_3_challenge_2(config: &Config) -> Result<i128, Error> {
    let file_buffer = open_puzzle_file_to_buf(config).unwrap();
    let lines = read_buf_into_vec_strings(file_buffer).unwrap();
    let mut priority_count: i128 = 0;
    let priority_lookup_map = generate_priority_lookup_map();
    let tri_false: (bool, bool, bool) = (false, false, false);
    let mut items_group_carried: HashMap<char, (bool, bool, bool)> = HashMap::new();

    let mut group_sequence: u8 = 0;
    for line in lines.into_iter() {
        //  Rust doesn't have a native way to index a tuple with a variable, so with 3
        //  iterations, we're just repeating code. :-(  There is apparently a crate
        //  that can be imported to index tuples, but oh well.
        if group_sequence == 0 {
            items_group_carried.clear();
            for item in line.chars() {
                let mut items_bagged = tri_false;
                items_bagged.0 |= true;
                items_group_carried.insert(item, items_bagged);
            }
            group_sequence += 1;
            continue;
        }
        if group_sequence == 1 {
            for item in line.chars() {
                if let Some(items_bagged_some) = items_group_carried.get(&item) {
                    let mut items_bagged = *items_bagged_some;
                    items_bagged.1 |= true;
                    items_group_carried.insert(item, items_bagged);
                } else {
                    let mut items_bagged = tri_false;
                    items_bagged.1 |= true;
                    items_group_carried.insert(item, items_bagged);
                }
            }
            group_sequence += 1;
            continue;
        }
        if group_sequence == 2 {
            for item in line.chars() {
                if let Some(items_bagged_some) = items_group_carried.get(&item) {
                    let mut items_bagged = *items_bagged_some;
                    items_bagged.2 |= true;
                    items_group_carried.insert(item, items_bagged);
                } else {
                    let mut items_bagged = tri_false;
                    items_bagged.2 |= true;
                    items_group_carried.insert(item, items_bagged);
                }
            }
            group_sequence += 1;
            // Don't continue loop, need to calculate the group now.
        }
        if group_sequence == 3 {
            // dbg!(&items_group_carried);
            group_sequence = 0;
            for (item, items_bagged) in items_group_carried.drain() {
                if items_bagged == (true, true, true) {
                    priority_count += *priority_lookup_map.get(&item).unwrap() as i128;
                    continue;
                }
            }
        }
    }
    Ok(priority_count)
}

pub fn test_config_d3() -> Config {
    let test_config: Config = Config {
        challenge: 3,
        filename: "./input/test3.txt".to_string(),
    };
    test_config
}

#[cfg(test)]
mod test {
    use super::*;
    test_expected_computed!(3, 1, test_config_d3());
    test_expected_computed!(3, 2, test_config_d3());
}
