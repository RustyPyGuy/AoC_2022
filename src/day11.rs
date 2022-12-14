// Advent of Code Day ELEVEN
#![allow(dead_code)]
use aoc_22::*;
use serde::Deserialize;
use std::collections::VecDeque;
use std::io::Error; //import custom lib.rs module

const DAY: u8 = 11;
const TEST1_EXPECTED_OUTPUT: &str = "10605";
const TEST2_EXPECTED_OUTPUT: &str = "0";

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(default)]
pub struct Monkey {
    #[serde(default, rename = "Number")]
    id: usize,
    #[serde(default, rename = "Starting items")]
    items: VecDeque<usize>,
    #[serde(default, rename = "Operation")]
    operation_string: String,
    operation: (char, usize), // operation and number (add, multiply, double self)
    #[serde(default, rename = "Test")]
    test_value: usize,
    #[serde(default, rename = "If true")]
    test_true: usize,
    #[serde(default, rename = "If false")]
    test_false: usize,
    total_inspections: usize,
}
impl Default for Monkey {
    fn default() -> Monkey {
        Monkey {
            id: 0,
            items: VecDeque::new(),
            operation: ('_', 0),
            operation_string: "".to_string(),
            test_value: 0,
            test_true: 0,
            test_false: 0,
            total_inspections: 0,
        }
    }
}

fn read_serialize_puzzle_file(config: &Config) -> Result<Vec<Monkey>, serde_yaml::Error> {
    let mut input_string = read_into_string(config);
    let mut reformatted_input_string: String = String::new();
    // This loop formats the input into compliant YAML
    // Why did I waste all of this time?  To get more experienced with Serde and YAML.
    for line in input_string.as_mut().lines() {
        let mut string_line = line.to_string();
        if string_line.contains("Starting items") {
            string_line = string_line
                .trim_start()
                .replace("Starting items:", "  Starting items: [");
            string_line.push_str(" ]");
        } else if string_line.trim_start().contains("Monkey") {
            string_line = string_line
                .trim_start()
                .replace("Monkey", "- Monkey:\n  Number:");
            string_line = string_line.trim_start().trim_end_matches(":").to_owned();
        } else if string_line.trim_start().contains("Operation") {
            string_line = string_line.trim_start().replace("Operation", "  Operation");
        } else if string_line.trim_start().contains("Test") {
            string_line = string_line
                .trim_start()
                .replace("Test: divisible by", "  Test:");
        } else if string_line.trim_start().contains("If true") {
            string_line = string_line
                .trim_start()
                .replace("If true: throw to monkey", "  If true:");
        } else if string_line.trim_start().contains("If false") {
            string_line = string_line
                .trim_start()
                .replace("If false: throw to monkey", "  If false:");
        } else {
            string_line = string_line.trim_start().to_owned();
            string_line.push_str("  ");
        }
        reformatted_input_string.push_str(string_line.as_str());
        reformatted_input_string.push('\n');
    }
    // Here's where deserialization with serde happens, but we're actually not done with our data structure yet.
    let mut monkey_vec: Vec<Monkey> = serde_yaml::from_str(&reformatted_input_string)?;
    for mut monkey in monkey_vec.iter_mut() {
        let mut temp_vec: Vec<&str> = monkey
            .operation_string
            .as_str()
            .split_whitespace()
            .collect();
        let num: usize = temp_vec.pop().unwrap().parse().unwrap_or(0);
        let mut op: char = temp_vec.pop().unwrap().parse().unwrap();
        // finishing with the data structure with decodable symbols.  An enum would be good here
        // too, but this is almost too simple.
        #[rustfmt::skip]
        match op {
            '+' => { op = 'a'; } // Add
            '*' => { op = 'm'; } // multiply
            _ => { panic!("Bad operation assignment."); }
        };
        if num == 0 {
            op = 'd';
        } // double
        monkey.operation = (op, num);
    }
    Ok(monkey_vec)
}

fn single_monkey_inspect(monkey_vec: &mut Vec<Monkey>, index: usize) {
    loop {
        if let Some(mut item) = monkey_vec[index].items.pop_front() {
            #[rustfmt::skip]
            match monkey_vec[index].operation.0 {
                'a' => { item = item + monkey_vec[index].operation.1; }
                'm' => { item = item * monkey_vec[index].operation.1; }
                'd' => { item = item * item; }
                _ => { panic!("Bad match."); }
            };

            item = item / 3;
            if item % monkey_vec[index].test_value == 0 {
                let action = monkey_vec[index].test_true;
                monkey_vec[action].items.push_back(item);
            } else {
                let action = monkey_vec[index].test_false;
                monkey_vec[action].items.push_back(item);
            }
            monkey_vec[index].total_inspections += 1;
        } else {
            break;
        }
    }
}

pub fn day_11_challenge_1(config: &Config) -> Result<i128, Error> {
    let mut monkey_vec: Vec<Monkey> = read_serialize_puzzle_file(config).unwrap();
    let monkey_len = monkey_vec.len().clone();
    for _ in 0..20 {
        // this approach taken because we don't want to actually iterate over the vector this time.
        for i in 0..monkey_len {
            single_monkey_inspect(&mut monkey_vec, i);
        }
    }
    let mut hi_1: usize = 0;
    let mut hi_2: usize = 0;
    for monkey in monkey_vec.iter() {
        if monkey.total_inspections > hi_1 {
            hi_2 = hi_1;
            hi_1 = monkey.total_inspections;
        } else if monkey.total_inspections > hi_2 {
            hi_2 = monkey.total_inspections;
        }
    }
    let monkey_business = hi_1 * hi_2;

    Ok(monkey_business as i128)
}

pub fn day_11_challenge_2(config: &Config) -> Result<i128, Error> {
    let buf = open_puzzle_file_to_buf(config).unwrap();
    let _vec_strings = read_buf_into_vec_strings(buf);
    Ok(1)
}

pub fn test_config_d11() -> Config {
    let test_config: Config = Config {
        challenge: 0,
        filename: "./input/test11.txt".to_string(),
    };
    test_config
}

#[cfg(test)]
mod test {
    use super::*;
    test_expected_computed!(11, 1, test_config_d11());
    test_expected_computed!(11, 2, test_config_d11());
}
