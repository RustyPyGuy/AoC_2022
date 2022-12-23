// Advent of Code Day FOURTEEN
//
// Some imports not needed every time.
#![allow(dead_code)]
use aoc_22::*;
use grid::Grid;
use itertools::Itertools;
use std::io::Error; //import custom lib.rs module

const DAY: u8 = 14;
const TEST1_EXPECTED_OUTPUT: &str = "24";
const TEST2_EXPECTED_OUTPUT: &str = "93";
// use std::process;

type Coord = (usize, usize);

pub fn day_14_challenge_1(config: &Config) -> Result<i128, Error> {
    let buf = open_puzzle_file_to_buf(config).unwrap();
    let vec_strings = read_buf_into_vec_strings(buf).unwrap();
    let coord_paths = read_and_parse_input(vec_strings);
    let x1_x2_y1_y2 = minima_maxima(&coord_paths);
    // find minima and maxima
    let x_offset = x1_x2_y1_y2.0 - 2; // This is an optimization to reduce the size of the matrix
                                      // since all values start at 500.
    let x_max = x1_x2_y1_y2.1 + 4 - x_offset;
    let y_max = x1_x2_y1_y2.1 + 2;
    // build rock matrix
    let mut rock_structure: Grid<bool> = Grid::new(y_max, x_max);
    let mut sand_structure: Grid<bool> = Grid::new(y_max, x_max);
    // the rock regolith
    build_rocks(coord_paths, &mut rock_structure, x_offset, false);
    // The falling sands.
    sand_grain_travel(&rock_structure, &mut sand_structure, x_offset);
    let mut sand_count: usize = 0;
    for grain in sand_structure.iter() {
        if *grain == true {
            sand_count += 1;
        }
    }
    Ok(sand_count as i128)
}

pub fn day_14_challenge_2(config: &Config) -> Result<i128, Error> {
    let buf = open_puzzle_file_to_buf(config).unwrap();
    let vec_strings = read_buf_into_vec_strings(buf).unwrap();
    let coord_paths = read_and_parse_input(vec_strings);
    let x1_x2_y1_y2 = minima_maxima(&coord_paths);
    // find minima and maxima
    let x_offset = 0; // we need lots of space, so this optimization is less so important.
                      // initial grid was encoutering bounds errors, so let's add 2 more!
    let x_max = 2 * x1_x2_y1_y2.1 + 4 - x_offset;
    let y_max = x1_x2_y1_y2.3 + 3; // small index adjustment
                                   // build rock matrix
    let mut rock_structure: Grid<bool> = Grid::new(y_max, x_max);
    let mut sand_structure: Grid<bool> = Grid::new(y_max, x_max);
    // the rocks
    build_rocks(coord_paths, &mut rock_structure, x_offset, true);
    // The falling sands.
    sand_grain_travel_c2(&rock_structure, &mut sand_structure, x_offset);
    let mut sand_count: usize = 0;
    for grain in sand_structure.iter() {
        if *grain == true {
            sand_count += 1;
        }
    }
    Ok(sand_count as i128)
}

fn read_and_parse_input(vec_of_strings: Vec<String>) -> Vec<Vec<Coord>> {
    let mut coord_paths: Vec<Vec<Coord>> = Vec::new();
    for vec in vec_of_strings.into_iter() {
        let mut vec_parsed: Vec<Coord> = Vec::new();
        let mut temp_item: Coord = (0, 0);
        let vec_delimited: Vec<&str> = vec.as_str().rsplit("->").collect();
        for item in vec_delimited.into_iter() {
            let mut vec_delimited1: Vec<&str> = item.trim().rsplit(',').collect();
            temp_item.0 = vec_delimited1.pop().unwrap().parse::<usize>().unwrap();
            temp_item.1 = vec_delimited1.pop().unwrap().parse::<usize>().unwrap();
            vec_parsed.push(temp_item);
        }
        coord_paths.push(vec_parsed);
    }
    coord_paths
}

fn minima_maxima(coord_paths: &Vec<Vec<Coord>>) -> (usize, usize, usize, usize) {
    // tuple: min_x, max_x, min_y, max_y
    let mut min_max_x1_x2_y1_y2: (usize, usize, usize, usize) = (500, 500, 0, 0);
    for item in coord_paths.iter().flatten() {
        if item.0 < min_max_x1_x2_y1_y2.0 {
            min_max_x1_x2_y1_y2.0 = item.0;
        } // minimum x coordinate
        if item.0 > min_max_x1_x2_y1_y2.1 {
            min_max_x1_x2_y1_y2.1 = item.0;
        } // maximum x coordinate
        if item.1 > min_max_x1_x2_y1_y2.3 {
            min_max_x1_x2_y1_y2.3 = item.1;
        } // maximum y coordinate (min is always 0)
    }
    min_max_x1_x2_y1_y2
}

fn build_rocks(
    input_coord_paths: Vec<Vec<Coord>>,
    rock_structure: &mut Grid<bool>,
    x_offset: usize,
    endless_floor: bool,
) {
    for rock_line in input_coord_paths.into_iter() {
        let mut rock_iter = rock_line.into_iter().multipeek();
        loop {
            let rock = rock_iter.next().unwrap_or((0, 0));
            let next_rock = rock_iter.peek().unwrap_or(&(0, 0));
            if next_rock == &(0, 0) {
                break;
            }
            rock_structure[rock.1][rock.0 - x_offset] = true;
            // lots of tests to deal with reverse ordering and stuff.
            if rock.0 < next_rock.0 {
                for x in rock.0 - x_offset..=next_rock.0 - x_offset {
                    rock_structure[rock.1][x] = true;
                }
            } else if rock.0 > next_rock.0 {
                for x in next_rock.0 - x_offset..=rock.0 - x_offset {
                    rock_structure[rock.1][x] = true;
                }
            } else if rock.0 == next_rock.0 {
                if rock.1 < next_rock.1 {
                    for y in rock.1..=next_rock.1 {
                        rock_structure[y][rock.0 - x_offset] = true;
                    }
                } else if rock.1 > next_rock.1 {
                    for y in next_rock.1..=rock.1 {
                        rock_structure[y][rock.0 - x_offset] = true;
                    }
                }
            }
        }
        let last_row: usize = rock_structure.rows() - 1;
        if endless_floor {
            for x in 0..rock_structure.cols() {
                rock_structure[last_row][x] = true;
            }
        }
    }
}

fn sand_grain_travel(rocks: &Grid<bool>, sands: &mut Grid<bool>, x_offset: usize) -> bool {
    'piling_sands: loop {
        let mut sand: Coord = (500, 0);
        'falling_grains: loop {
            if sand.1 >= sands.rows() - 1 {
                break 'piling_sands;
            }
            // let sand_rock = rocks[sand.1][sand.0 - x_offset] || sands[sand.1][sand.0 - x_offset];
            let sand_rock_d =
                rocks[sand.1 + 1][sand.0 - x_offset] || sands[sand.1 + 1][sand.0 - x_offset];
            let sand_rock_l = rocks[sand.1 + 1][sand.0 - x_offset - 1]
                || sands[sand.1 + 1][sand.0 - x_offset - 1];
            let sand_rock_r = rocks[sand.1 + 1][sand.0 - x_offset + 1]
                || sands[sand.1 + 1][sand.0 - x_offset + 1];
            #[rustfmt::skip]
                match (sand_rock_l, sand_rock_d, sand_rock_r) {
                    (_, false, _) => { sand.1 += 1; } //falling down
                    (false, true, _) => { sand.1 += 1; sand.0 -= 1; } //falling left
                    (true, true, false) => { sand.1 += 1; sand.0 += 1; } //falling right
                    (true, true, true) => { sands[sand.1][sand.0 - x_offset] = true;
                        break 'falling_grains; } // coming to rest
                    // (_, _, _) => { panic!("impossible state."); } // bug catcher
                };
        }
    }
    true
}
fn sand_grain_travel_c2(rocks: &Grid<bool>, sands: &mut Grid<bool>, x_offset: usize) -> bool {
    'piling_sands: loop {
        let mut sand: Coord = (500, 0);
        'falling_grains: loop {
            let sand_rock_d: bool;
            let sand_rock_l: bool;
            let sand_rock_r: bool;
            if sand.1 > sands.rows() - 1 {
                break 'piling_sands;
            }
            let sand_rock = rocks[sand.1][sand.0 - x_offset] || sands[sand.1][sand.0 - x_offset];
            if sand_rock == true {
                break 'piling_sands;
            }
            if sand.1 == sands.rows() - 1 {
                sand_rock_d = true;
                sand_rock_l = true;
                sand_rock_r = true;
            } else {
                sand_rock_l = rocks[sand.1 + 1][sand.0 - x_offset - 1]
                    || sands[sand.1 + 1][sand.0 - x_offset - 1];
                sand_rock_d =
                    rocks[sand.1 + 1][sand.0 - x_offset] || sands[sand.1 + 1][sand.0 - x_offset];
                sand_rock_r = rocks[sand.1 + 1][sand.0 - x_offset + 1]
                    || sands[sand.1 + 1][sand.0 - x_offset + 1];
            }
            #[rustfmt::skip]
                match (sand_rock_l, sand_rock_d, sand_rock_r) {
                    (_, false, _) => { sand.1 += 1; } //falling down
                    (false, true, _) => { sand.1 += 1; sand.0 -= 1; } //falling left
                    (true, true, false) => { sand.1 += 1; sand.0 += 1; } //falling right
                    (true, true, true) => { sands[sand.1][sand.0 - x_offset] = true;
                        break 'falling_grains; } // coming to rest
                    // (_, _, _) => { panic!("impossible state."); } // bug catcher
                };
        }
    }
    true
}

pub fn test_config_d14() -> Config {
    let test_config: Config = Config {
        challenge: 0,
        filename: "./input/test14.txt".to_string(),
    };
    test_config
}

#[cfg(test)]
mod test {
    use super::*;
    test_expected_computed!(14, 1, test_config_d14());
    test_expected_computed!(14, 2, test_config_d14());
}
