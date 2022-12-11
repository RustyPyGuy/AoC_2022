// Advent of Code Day NINE 😄
//
// Some imports not needed every time.
#![allow(dead_code)]
use aoc_22::*;
use std::io::Error; //import custom lib.rs module

const DAY: u8 = 9;
const TEST1_EXPECTED_OUTPUT: &str = "13";
const TEST2_EXPECTED_OUTPUT: &str = "36";
use std::collections::HashSet;

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
pub struct Coord(isize, isize);

pub fn day_9_challenge_1(config: &Config) -> Result<i128, Error> {
    let buf = open_puzzle_file_to_buf(config).unwrap();
    let vec_strings = read_buf_into_vec_strings(buf).unwrap();
    let mut current_head = Coord(0, 0);
    let mut current_tail = Coord(0, 0);
    let mut coord_history: HashSet<Coord> = HashSet::new();
    coord_history.insert(Coord(0, 0)); // overlap at the starting point
    for string in vec_strings.into_iter() {
        let direction;
        let segments: Vec<&str> = string.split_whitespace().collect();
        #[rustfmt::skip]
        match segments[0] {
            "U" => { direction = Direction::Up; }
            "D" => { direction = Direction::Down; }
            "L" => { direction = Direction::Left; }
            "R" => { direction = Direction::Right; }
            _ => { direction = Direction::None; }
        };
        let distance = segments[1].parse::<isize>().unwrap();
        current_head = set_position(current_head, &direction, &distance).unwrap();
        let tail_out = calc_tail_position(&current_head, current_tail).unwrap();
        for coord in tail_out.1.into_iter() {
            coord_history.insert(coord);
        }
        current_tail = tail_out.0;
    }
    return Ok(coord_history.len() as i128);

    // private function to this challenge
    fn calc_tail_position(
        head: &Coord,
        mut tail: Coord,
    ) -> Result<(Coord, Vec<Coord>), &'static str> {
        // I considered borrowing a mutable reference to modify the tail variable, but that was
        // more code rework than I wanted to bother with right now. It could improve performance.
        let head_final_coord = *head; // Coord = set_position(*head, &direction, &distance).unwrap();
        let mut tail_final_coord: Coord = tail;
        let mut tail_walk_vec: Vec<Coord> = Vec::new();
        if head_final_coord == tail {
            tail_walk_vec.push(tail);
            return Ok((tail_final_coord, tail_walk_vec));
        }
        let mut breakme: isize = 50;
        'z: loop {
            if breakme < 1 {
                break 'z;
            }
            if let Some(tail_) = walk_knot_one_unit(*head, tail) {
                tail_final_coord = tail_;
                tail = tail_;
                tail_walk_vec.push(tail_final_coord);
            } else {
                break 'z;
            }
            breakme -= 1;
        }
        Ok((tail_final_coord, tail_walk_vec))
    }
}

pub fn day_9_challenge_2(config: &Config) -> Result<i128, Error> {
    let buf = open_puzzle_file_to_buf(config).unwrap();
    let vec_strings = read_buf_into_vec_strings(buf).unwrap();
    let mut current_head = Coord(0, 0);
    let mut coord_history: HashSet<Coord> = HashSet::new();
    #[allow(unused_assignments)]
    let mut knot9_list: Vec<Coord> = Vec::with_capacity(8);
    let mut rope: Vec<Coord> = vec![Coord(0, 0); 10]; // rope vector elements 0 - 9 head to tail
    coord_history.insert(Coord(0, 0)); // overlap at the starting point
    for string in vec_strings.into_iter() {
        let direction: Direction;
        let segments: Vec<&str> = string.split_whitespace().collect();
        #[rustfmt::skip]
        match segments[0] {
            "U" => { direction = Direction::Up; }
            "D" => { direction = Direction::Down; }
            "L" => { direction = Direction::Left; }
            "R" => { direction = Direction::Right; }
            _ => { direction = Direction::None; }
        };
        let distance = segments[1].parse::<isize>().unwrap();
        current_head = set_position(current_head, &direction, &distance).unwrap();
        (rope, knot9_list) = calc_tail_position_c2(&current_head, &rope).unwrap();
        for knot9 in knot9_list.into_iter() {
            coord_history.insert(knot9);
        }
    }

    // private function to this challenge is different than challenge 1
    fn calc_tail_position_c2(
        head_dest: &Coord,
        rope_vec: &Vec<Coord>,
    ) -> Result<(Vec<Coord>, Vec<Coord>), &'static str> {
        // output: tail coordinate, rope_vector, tail_history_vector
        // I considered borrowing a mutable reference to modity the tail variable, but that was
        // more code rework than I wanted to bother with right now.
        let mut tail_walk_vec: Vec<Coord> = Vec::new();
        let mut rope_vec_calc: Vec<Coord> = rope_vec.clone();
        'rope: loop {
            'knot: for i in 0..=9 {
                let rope_vec_update: Coord;
                // follow the knots
                let iter_lead_knot;
                if i == 0 {
                    rope_vec_calc[i] = walk_knot_one_unit(*head_dest, rope_vec_calc[i])
                        .unwrap_or(rope_vec_calc[i]);
                    continue 'knot;
                } else {
                    iter_lead_knot = rope_vec_calc[i - 1];
                }

                if let Some(knot_) = walk_knot_one_unit(iter_lead_knot, rope_vec_calc[i]) {
                    rope_vec_update = knot_;
                } else {
                    rope_vec_update = rope_vec_calc[i]; // uncecesary assignment? should be no change
                }
                rope_vec_calc[i] = rope_vec_update;
                if i == 9 {
                    tail_walk_vec.push(rope_vec_update); // push a rope, heh.
                }
                rope_vec_calc[i] = rope_vec_update;
            }
            if rope_vec_calc[0] == *head_dest {
                break 'rope;
            } else if walk_knot_one_unit(*head_dest, rope_vec_calc[0]) == None {
                rope_vec_calc[0] = *head_dest;
            }
        }
        return Ok((rope_vec_calc, tail_walk_vec));
    }
    return Ok(coord_history.len() as i128);
}

#[rustfmt::skip]
#[derive(Debug)]
pub enum Direction { Up, Down, Left, Right, None }

fn set_position(
    // to set a new head position
    point: Coord,
    direction: &Direction,
    distance: &isize,
) -> Result<Coord, &'static str> {
    let mut point_final_coord: Coord = point;
    #[rustfmt::skip]
    match direction {
        Direction::Right => { point_final_coord.0 += distance; }
        Direction::Left => { point_final_coord.0 -= distance; }
        Direction::Up => { point_final_coord.1 += distance; }
        Direction::Down => { point_final_coord.1 -= distance; }
        _ => { return Err("Bad Direction") }
    };
    Ok(point_final_coord)
}

fn walk_knot_one_unit(head: Coord, mut tail: Coord) -> Option<Coord> {
    #[rustfmt::skip]
    match ((head.0 - tail.0).abs(), (head.1 - tail.1).abs()) {
        (0,0) => { return None; }, // no change to tail
        (1,0) => { return None; }, // no change to tail
        (0,1) => { return None; }, // no change to tail
        (1,1) => { return None; }, // no change to tail
        (0,2..) => { tail.1 = tail.1 + (head.1 - tail.1).signum(); }, // only y movement
        (2..,0) => { tail.0 = tail.0 + (head.0 - tail.0).signum(); }, // only x movement
        (1,2..) => { tail.0 = head.0;  // diagonal movement with long y component
                tail.1 = tail.1 + (head.1 - tail.1).signum(); }
        (2..,1) => {tail.0 = tail.0 + (head.0 - tail.0).signum(); // diagonal movement with long x component
                tail.1 = head.1; }
        (2,2) => {tail.0 = tail.0 + (head.0 - tail.0).signum(); // double diagonal movement that only happens in a longer rope where two knots are touching diagonally.
                tail.1 = tail.1 + (head.1 - tail.1).signum();}
        (_,_) => { panic!("bad coordinate request. head: {:?}, tail {:?} ", head, tail); }
    };
    Some(tail)
}

// Test setup.  Two different test inputs.
pub fn test_config_d9() -> Config {
    let test_config: Config = Config {
        challenge: 0,
        filename: "./input/test9.txt".to_string(),
    };
    test_config
}

pub fn test_config_d9_2() -> Config {
    let test_config: Config = Config {
        challenge: 0,
        filename: "./input/test9_2.txt".to_string(),
    };
    test_config
}

#[cfg(test)]
mod test {
    use super::*;
    test_expected_computed!(9, 1, test_config_d9());
    test_expected_computed!(9, 2, test_config_d9_2());
}
