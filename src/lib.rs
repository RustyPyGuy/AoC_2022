// Advent of Code 2022!
// Structs and methods used for each or most days are included in this lib.rs file
// This includes things like opening files and reading input.
// #![feature(trace_macros)]
use std::fs;
use std::fs::File;
// use std::error::Error;
use clap::Parser;
// use paste;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};
use std::process;

#[derive(Parser, Debug, Clone)]
#[clap(author, version, about, long_about = None)]
pub struct CLIArgs {
    /// [WIP] Verbose console output.
    #[clap(short, long, action)]
    pub verbose: bool,

    /// The day to run a challenge automatically. Default of 0 is for test.
    #[clap(short, long, value_parser, default_value_t = 0)]
    pub day: u8,
}

pub struct Config {
    pub challenge: u8,
    pub filename: String,
    // pub case_sensitive: bool,
}

impl Config {
    pub fn parse_config(args: CLIArgs) -> Result<Config, Error> {
        let challenge: u8 = args.day;
        let mut filename: String = String::with_capacity(20);
        if args.day < 10 {
            filename = format!("input/input0{}.txt", args.day).to_string();
        }
        if args.day >= 10 {
            filename = format!("input/input{}.txt", args.day).to_string();
        }
        if filename.is_empty() {
            return Err(Error::new(ErrorKind::Other, "oh no!"));
        }
        Ok(Config {
            challenge,
            filename,
        })
    }
}

pub fn open_puzzle_file_to_buf(config: &Config) -> Result<std::io::BufReader<File>, Error> {
    let f = File::open(config.filename.clone()).unwrap_or_else(|err| {
        println!("Error opening file: {}", err);
        process::exit(1);
    });
    let br = BufReader::new(f);
    Ok(br)
}

pub fn read_buf_into_vec_lines<R: Read>(io: R) -> Result<Vec<i32>, std::io::Error> {
    let br = BufReader::new(io);
    let mut v = Vec::<i32>::with_capacity(2048);
    for line in br.lines() {
        if line.as_ref().unwrap().is_empty() {
            v.push(0);
            continue;
        }
        v.push(
            line?
                .trim()
                .parse()
                .map_err(|e| std::io::Error::new(ErrorKind::InvalidData, e))?,
        );
    }
    Ok(v)
}

pub fn read_buf_into_vec_strings<R: Read>(io: R) -> Result<Vec<String>, std::io::Error> {
    let br = BufReader::new(io);
    let mut v = Vec::<String>::with_capacity(2048);
    for line in br.lines() {
        if line.as_ref().unwrap().is_empty() {
            continue;
        }
        v.push(line.map_err(|e| std::io::Error::new(ErrorKind::InvalidData, e))?);
    }
    Ok(v)
}

pub fn read_into_string(config: &Config) -> String {
    // let f = File::open(config.filename.clone()).unwrap_or_else(|err| {
    //     println!("Error opening file: {}", err);
    //     process::exit(1);
    // });
    let file_contents = fs::read_to_string(config.filename.clone()).unwrap();
    file_contents
}

pub fn read_string_to_vec_lines<'a>(input: &'a String) -> Vec<&'a str> {
    let lines: Vec<&'a str> = input.lines().filter(|l| l != &"").collect();
    // let output = lines.iter();
    // output
    lines
}
// pub fn iterate_by_lines_from_string(input: String, output: Vec<&'static mut str>) -> Vec<&'static mut str> {
//     let mut lines: Vec<&'static mut str> = input.lines().filter(|l| l != &"").collect();
//     // let output = lines.iter();
//     // output
//     output = lines;
//     output
// }
pub fn parse_string_into_vec_integers(input: String, delimiter: char) -> Vec<u32> {
    // let lines: Vec<&str> = input.lines().filter(|l| l != &"").collect();
    let numbers: Vec<u32> = input
        .split(&delimiter.to_string())
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    numbers
}
pub fn parse_string_into_integers_usize(input: String, delimiter: char) -> Vec<usize> {
    // let lines: Vec<&str> = input.lines().filter(|l| l != &"").collect();
    let numbers: Vec<usize> = input
        .split(&delimiter.to_string())
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    numbers
}

#[macro_export]
macro_rules! test_expected_computed {
    ($day:literal, $challenge:literal, $config:expr) => {
paste::item! {
            #[test]
            fn [< compare_output_example_day_ $day _challenge_ $challenge >] () {
            let computed_output = [< day_ $day _challenge_ $challenge >] (&$config);
            println!(
                "Expected output: {}\nComputed output: {}\n\n",
                & [< TEST $challenge _EXPECTED_OUTPUT>] ,
                &computed_output.as_ref().unwrap()
            );
            if [<TEST $challenge _EXPECTED_OUTPUT>] .parse::<i128>().unwrap() == computed_output.unwrap() {
                assert!(true);
            } else {
                assert!(false);
            }
        }
            }
    };
}
