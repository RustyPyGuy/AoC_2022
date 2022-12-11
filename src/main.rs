/*
 * Advent of Code 2022 - RustyPyGuy
 *
 */
use aoc_22::*; //import lib module
#[allow(unused_imports)]
use std::time::{Duration, Instant};
mod day0;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
// 8
mod day9;

use clap::Parser;
// use std::env;
use std::process;

fn main() {
    let start = Instant::now();
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
        4 => {
            result.push(
                day4::day_4_challenge_1(&config.as_ref().unwrap())
                    .unwrap()
                    .into(),
            );
            result.push(
                day4::day_4_challenge_2(&config.as_ref().unwrap())
                    .unwrap()
                    .into(),
            );
        }
        5 => {
            let output1: String = day5::day_5_challenge_1(&config.as_ref().unwrap()).unwrap();
            let output2: String = day5::day_5_challenge_2(&config.as_ref().unwrap()).unwrap();
            println!("output 1:  {}\noutput 2:  {}\n\n", output1, output2);
        }
        6 => {
            result.push(
                day6::day_6_challenge_1(&config.as_ref().unwrap())
                    .unwrap()
                    .into(),
            );
            result.push(
                day6::day_6_challenge_2(&config.as_ref().unwrap())
                    .unwrap()
                    .into(),
            );
        }
        7 => {
            result.push(
                day7::day_7_challenge_1(&config.as_ref().unwrap())
                    .unwrap()
                    .into(),
            );
            result.push(
                day7::day_7_challenge_2(&config.as_ref().unwrap())
                    .unwrap()
                    .into(),
            );
        }
        9 => {
            result.push(
                day9::day_9_challenge_1(&config.as_ref().unwrap())
                    .unwrap()
                    .into(),
            );
            result.push(
                day9::day_9_challenge_2(&config.as_ref().unwrap())
                    .unwrap()
                    .into(),
            );
        }
        10..=25 => {
            println!("These challenges haven't been completed yet.  Try again later!!");
            process::exit(1);
        }

        _ => {
            println!("Invalid challenge input. Exiting");
            process::exit(1);
        }
    };
    let duration = start.elapsed();
    println!(
        "\nThe results for Day {} are:\n\
        Challenge 1 result {}\nChallenge 2 result {}\n\nExecution time including file reading: {:?}",
        config.unwrap().challenge,
        result[0],
        result[1],
duration
    );
}
