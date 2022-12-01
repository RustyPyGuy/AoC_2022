/*
 * Advent of Code 2022 - RustyPyGuy
 *
 */
use aoc_22::*; //import lib module
mod day0;
// mod day1;

use clap::Parser;
// use std::env;
use std::process;

fn main() {
    println!("Advent of Code!");
    // parse command line
    let cli_args = CLIArgs::parse();
    if cli_args.verbose {
        // do stuff
    }
    let mut result: Vec<i128> = Vec::new();
    let config = aoc_22::Config::parse_config(cli_args.clone());
    match cli_args.day {
        0 => {
            result.push(
                day0::day_0_challenge_1(&config.as_ref().unwrap())
                    .unwrap()
                    .into(),
            );
            result.push(
                day0::day_0_challenge_2(&config.as_ref().unwrap())
                    .unwrap()
                    .into(),
            );
        }

        // 1 => {
        //     result.push(day1_challege1(&config).unwrap().into());
        //     result.push(day1_challenge2(&config).unwrap().into());
        // }
        // 2 => {
        //     result.push(day2_challenge1(&config).unwrap().into());
        //     result.push(day2_challenge2(&config).unwrap().into());
        // }
        // 3 => {
        //     result.push(day3_challenge1(&config).unwrap().into());
        //     result.push(day3_challenge2(&config).unwrap().into());
        // }
        // 4 => {
        //     result.push(day4_challenge1(&config).unwrap().into());
        //     result.push(day4_challenge2(&config).unwrap().into());
        // }
        // 5 => {
        //     result.push(day5_challenge1(&config).unwrap().into());
        //     result.push(day5_challenge2(&config).unwrap().into());
        // }
        // 6 => {
        //     result.push(day6_challenge1(&config).unwrap().into());
        //     result.push(day6_challenge2(&config).unwrap().try_into().unwrap());
        // }
        // 7 => {
        //     result.push(day7_challenge1(&config).unwrap());
        //     result.push(day7_challenge2(&config).unwrap());
        // }
        // 8 => {
        //     result.push(day8_challenge1(&config).unwrap());
        //     result.push(day8_challenge2(&config).unwrap());
        // }
        // 9 => {
        //     result.push(day9_challenge1(&config).unwrap());
        //     result.push(day9_challenge2(&config).unwrap());
        // }
        // 11 => {
        //     result.push(day11_challenge1(&config).unwrap());
        //     result.push(day11_challenge2(&config).unwrap());
        // }
        _ => {
            println!("Invalid challenge input. Exiting");
            process::exit(1);
        }
    };
    println!(
        "The results for Day {} are:\n\
        Challenge 1 result {}\nChallenge 2 result {}",
        config.unwrap().challenge,
        result[0],
        result[1]
    );
}
