// Advent of Code Day TWO
//
// Some imports not needed every time.
#![allow(dead_code)]
use aoc_22::*;
use std::io::Error; //import custom lib.rs module

const DAY: u8 = 2;
const TEST1_EXPECTED_OUTPUT: &str = "15";
const TEST2_EXPECTED_OUTPUT: &str = "0";
// use std::process;

#[derive(Debug, Copy, Clone)]
#[allow(non_snake_case)]
pub struct RPS {
    A: u8,
    B: u8,
    C: u8,
    O1: u8,
    O2: u8,
    O3: u8,
}
impl RPS {
    pub fn calc_game(self: Self, (abc, xyz): (char, char)) -> (u8, u8) {
        // match abc
        let player1 = match abc {
            'A' => self.A,
            'B' => self.B,
            'C' => self.C,
            _ => u8::MAX, // will cause buffer overflow :-)
        };
        let player2 = match xyz {
            'X' => self.O1,
            'Y' => self.O2,
            'Z' => self.O3,
            _ => u8::MAX,
        };
        if player1 == 1 && player2 == 1 {
            // two rocks
            return (player1 + 3, player2 + 3);
        }
        if player1 == 1 && player2 == 2 {
            // rock paper
            return (player1, player2 + 6);
        }
        if player1 == 1 && player2 == 3 {
            // rock scissors
            return (player1 + 6, player2);
        }
        if player1 == 2 && player2 == 1 {
            // paper rock
            return (player1 + 6, player2);
        }
        if player1 == 2 && player2 == 2 {
            // paper paper
            return (player1 + 3, player2 + 3);
        }
        if player1 == 2 && player2 == 3 {
            // paper scissors
            return (player1, player2 + 6);
        }
        if player1 == 3 && player2 == 1 {
            //scissors rock
            return (player1, player2 + 6);
        }
        if player1 == 3 && player2 == 2 {
            // scissors paper
            return (player1 + 6, player2);
        }
        if player1 == 3 && player2 == 3 {
            // scissors scissors
            return (player1 + 3, player2 + 3);
        } else {
            return (0, 0);
        }
    }
}

pub fn day_2_challenge_1(config: &Config) -> Result<i128, Error> {
    let rps_challenge1 = RPS {
        A: 1, //rock
        B: 2, //paper
        C: 3, //scissors
        O1: 1,
        O2: 2,
        O3: 3,
    };
    let mut game_total_player2: usize = 0;
    let input_string: String = read_into_string(config);
    let vec_lines = read_string_to_vec_lines(&input_string);
    for line in vec_lines.into_iter() {
        // for chars_ in line.chars() {
        let mut splitline = line.split_whitespace();
        let game: (char, char) = (
            splitline.next().unwrap().chars().next().unwrap(),
            splitline.next().unwrap().chars().next().unwrap(),
        );
        game_total_player2 += rps_challenge1.calc_game(game).1 as usize;
        dbg!(game_total_player2);
    }
    Ok(game_total_player2 as i128)
}

pub fn day_2_challenge_2(config: &Config) -> Result<i128, Error> {
    Ok(0)
}

pub fn test_config_d2() -> Config {
    let test_config: Config = Config {
        challenge: 0,
        filename: "./input/test2.txt".to_string(),
    };
    test_config
}

#[cfg(test)]
mod test {
    use super::*;
    test_expected_computed!(2, 1, test_config_d2());
    test_expected_computed!(2, 2, test_config_d2());
}
