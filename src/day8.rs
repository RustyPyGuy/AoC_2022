// Advent of Code Day EIGHT
//
#![allow(dead_code)]
use aoc_22::*;
use grid::Grid;
use std::io::Error; //import custom lib.rs module
const DAY: u8 = 8;
const TEST1_EXPECTED_OUTPUT: &str = "21";
const TEST2_EXPECTED_OUTPUT: &str = "8";

// NOTE: unit tests fail if these constants are not set to 5 x 5.
const ROWS: usize = 99;
const COLS: usize = 99;

pub fn day_8_challenge_1(config: &Config) -> Result<i128, Error> {
    let buf = open_puzzle_file_to_buf(config).unwrap();
    let vec_strings = read_buf_into_vec_strings(buf).unwrap();

    let mut tree_map: Grid<u8> = Grid::new(ROWS, COLS);
    for (r, line) in vec_strings.into_iter().enumerate() {
        for (c, char_) in line.chars().enumerate() {
            tree_map[r][c] = char_.to_digit(10).unwrap().try_into().unwrap();
        }
    }
    let mut mask_grid: Grid<bool> = Grid::init(ROWS, COLS, false);
    for r in 0..ROWS {
        let mut row_peak = 0;
        for c in 0..COLS {
            if r == 0 {
                mask_grid[r][c] = true;
            }
            // Mark increases from each direction with a mask grid
            if tree_map[r][c] > row_peak {
                mask_grid[r][c] = true;
                row_peak = tree_map[r][c];
            }
        }
    }
    // run in reverse. Note column indexes
    for r in 0..ROWS {
        let mut row_peak = 0;
        for c in (0..COLS).rev() {
            if r == ROWS - 1 {
                mask_grid[r][c] = true;
            }
            // Mark increases from this direction to a mask grid?
            if tree_map[r][c] > row_peak {
                mask_grid[r][c] = true;
                row_peak = tree_map[r][c];
            }
        }
    }
    // transpose the grid because it is more efficient to iterate by column
    let tree_map = tree_map.transpose();
    let mut mask_grid = mask_grid.transpose();

    for r in 0..ROWS {
        let mut row_peak = 0;
        for c in 0..COLS {
            if r == 0 {
                mask_grid[r][c] = true;
            }
            // Mark increases from this direction to a mask grid?
            if tree_map[r][c] > row_peak {
                mask_grid[r][c] = true;
                row_peak = tree_map[r][c];
            }
        }
    }
    // run in reverse. Note column indexes
    for r in 0..ROWS {
        let mut row_peak = 0;
        for c in (0..COLS).rev() {
            if r == ROWS - 1 {
                mask_grid[r][c] = true;
            }
            // Mark increases from this direction to a mask grid?
            if tree_map[r][c] > row_peak {
                mask_grid[r][c] = true;
                row_peak = tree_map[r][c];
            }
        }
    }
    let mut tree_counter: usize = 0;
    for item in mask_grid.flatten().into_iter() {
        if *item == true {
            tree_counter += 1;
        }
    }
    Ok(tree_counter as i128)
}

pub fn day_8_challenge_2(config: &Config) -> Result<i128, Error> {
    let buf = open_puzzle_file_to_buf(config).unwrap();
    let vec_strings = read_buf_into_vec_strings(buf).unwrap();
    let mut tree_map: Grid<u8> = Grid::new(ROWS, COLS);
    for (r, line) in vec_strings.into_iter().enumerate() {
        for (c, char_) in line.chars().enumerate() {
            tree_map[r][c] = char_.to_digit(10).unwrap().try_into().unwrap();
        }
    }
    let mut best_score: usize = 0;
    for r in 1..ROWS - 1 {
        for c in 1..COLS - 1 {
            // Mark increases from this direction to a mask grid?
            let mut left_counter: usize = 0;
            for left in (0..c).rev() {
                if tree_map[r][left] >= tree_map[r][c] {
                    left_counter += 1;
                    break;
                } else {
                    left_counter += 1;
                }
            }
            let mut right_counter: usize = 0;
            for right in c + 1..COLS {
                if tree_map[r][right] >= tree_map[r][c] {
                    right_counter += 1;
                    break;
                } else {
                    right_counter += 1;
                }
            }
            let mut up_counter: usize = 0;
            for up in (0..r).rev() {
                if tree_map[up][c] >= tree_map[r][c] {
                    up_counter += 1;
                    break;
                } else {
                    up_counter += 1;
                }
            }
            let mut down_counter: usize = 0;
            for down in r + 1..ROWS {
                if tree_map[down][c] >= tree_map[r][c] {
                    down_counter += 1;
                    break;
                } else {
                    down_counter += 1;
                }
            }
            let tree_score = left_counter * right_counter * up_counter * down_counter;
            if tree_score > best_score {
                best_score = tree_score;
            }
        }
    }

    Ok(best_score as i128)
}

pub fn test_config_d8() -> Config {
    let test_config: Config = Config {
        challenge: 0,
        filename: "./input/test8.txt".to_string(),
    };
    test_config
}

#[cfg(test)]
mod test {
    use super::*;
    test_expected_computed!(8, 1, test_config_d8());
    test_expected_computed!(8, 2, test_config_d8());
}
