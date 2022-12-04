/*
 * Advent of Code 2022 - RustyPyGuy
 *
 */
use aoc_22::*; //import lib module
mod day0;
mod day1;
mod day2;
mod day3;

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
        1 => {
            result.push(
                day1::day_1_challenge_1(&config.as_ref().unwrap())
                    .unwrap()
                    .into(),
            );
            result.push(
                day1::day_1_challenge_2(&config.as_ref().unwrap())
                    .unwrap()
                    .into(),
            );
        }
        2 => {
            result.push(
                day2::day_2_challenge_1(&config.as_ref().unwrap())
                    .unwrap()
                    .into(),
            );
            result.push(
                day2::day_2_challenge_2(&config.as_ref().unwrap())
                    .unwrap()
                    .into(),
            );
        }
        3 => {
            result.push(
                day3::day_3_challenge_1(&config.as_ref().unwrap())
                    .unwrap()
                    .into(),
            );
            result.push(
                day3::day_3_challenge_2(&config.as_ref().unwrap())
                    .unwrap()
                    .into(),
            );
        }

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
