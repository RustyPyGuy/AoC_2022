// Advent of Code Day SIX
//
// silence the compiler from warning about unused variables and such.
#![allow(dead_code)]
// import my library function lib.rs
use aoc_22::*;
// import the Itertools crate
use itertools::Itertools;
// import HashMap fromt the standard libary
use std::collections::HashSet;
// Import error types.
use std::io::Error;

// constants used for my unit tests.  The unit tests read the sample input and compare to the
// expected output.  Those functions are defined in other files and are built with macros.
const DAY: u8 = 6;
const TEST1_EXPECTED_OUTPUT: &str = "11";
const TEST2_EXPECTED_OUTPUT: &str = "26";

/// function to read a config struct and return an enum type Result which is either Ok with an integer or an Error type.
pub fn day_6_challenge_1(config: &Config) -> Result<i128, Error> {
    // define a String variable from the file output. The called function defined in lib.rs
    let input_string = read_into_string(&config);
    // There's a lot that happens in this next line even though there are only two methods.
    // convert the string into an iterator of tuples that contain an index of the characters and
    // the actual characters and convert the type into an iterator type that supports "peeking" at
    // the values in the iterator beyond the current and next values.  The multipeek is provided by
    // an imported libarary called itertools. This variable is our main data structure.
    let mut string_multipeek = input_string.char_indices().multipeek();
    // ititialize and define a placeholder variable for the output.  This can't be empty or the
    // compilier will complain because it's the return value of the function. It's a tuple of the
    // index and a character.
    let mut output: (usize, char) = (0, '_');
    // initialize and construct an empty HashSet. The capacity helps pre-allocate memory but is not
    // a limit.
    let mut hash_set: HashSet<char> = HashSet::with_capacity(4);
    // start an infinite loop!  Loop exit will be done with break statements.
    loop {
        // clear the HashSet at the beginning of each loop
        hash_set.clear();
        // initialize  a temporary placeholder value.  It's defined later, but it needs to be
        // initialized in this exterior scope to live past it's definition in if statemenets.
        let iterant;
        // advance to the next iterant in data structure and consume it in the process.  Allow for
        // the fact that it may not exist.  Some means there is a value vs. None.
        // This is weird syntax and took me a while to get used to.  This can also be done with
        // normal if or match statements.
        if let Some(i) = string_multipeek.next() {
            // add the character element to the hash set (second index of the tuple)
            hash_set.insert(i.1);
            // set our placeholder variable to the current iterant.  This step is necessary because the
            // i value will go out of scope after the braces and we need to keep it.
            iterant = i;
            // The else condition is reached if there is no more data to read.  We should not reach
            // this condition as an answer should be discovered first.
        } else {
            break; // end of string is reached.
        }
        // look ahead "peek" to the next three variables and add them to the HashMap.  The "peek"
        // looks at next iterations of the iterator without advancing or consuming and does so
        // immutably, basically read-only access.
        for _ in 0..3 {
            // "Some" indicates there is a value, if the peek returns "None", then there is nothing
            // to insert in the HashMap.
            if let Some(i) = string_multipeek.peek() {
                hash_set.insert(i.1);
            }
        }
        // This next comparison expression is key.  If a value is already conatined in the HashMap, then it
        // will not be added and the HashMap will not grow in size, and the length of the HashMap
        // will not reach 4. If there are exactly 4 unique values, then the HashMap will contain 4
        // values and those are all guaranteed to be unique and we found our challenge criteria.
        if hash_set.len() == 4 {
            // the current iterated value is the start of the sequence and the output is 4 values
            // later.  Break out of the loop and return the answer.
            output = (iterant.0 + 4, iterant.1); // result index is the end of the peek
            break;
        }
    }
    // Returns the answer with the correct type as defined with an OK (note lack of semicolon to
    // return)
    Ok(output.0 as i128)
}

/// This function is largely the same as above with some values adjusted.
/// comments added for the differences.
pub fn day_6_challenge_2(config: &Config) -> Result<i128, Error> {
    let input_string = read_into_string(&config);
    let mut string_multipeek = input_string.char_indices().multipeek();
    let mut output: (usize, char) = (0, '_');
    let mut hash_set: HashSet<char> = HashSet::with_capacity(14); // larger pre-allocation.
    loop {
        hash_set.clear();
        let iterant;
        if let Some(i) = string_multipeek.next() {
            hash_set.insert(i.1);
            iterant = i;
        } else {
            break; // end of string
        }
        // peek at the next 13 and add to hashmap
        for _ in 0..13 {
            if let Some(i) = string_multipeek.peek() {
                if !hash_set.insert(i.1) {
                    // immediately break the loop if the HashSet fails to insert a value.
                    // This iteration of the loop is not a winner.
                    break;
                } // if there's a duplicate in the hashset, move through the loop
            }
        }
        if hash_set.len() == 14 {
            // if the hashmap has 14 elements, then there are no repeats
            output = (iterant.0 + 14, iterant.1); // result index is the end of the peek
            break;
        }
    }
    Ok(output.0 as i128)
}

pub fn test_config_d6() -> Config {
    let test_config: Config = Config {
        challenge: 0,
        filename: "./input/test6.txt".to_string(),
    };
    test_config
}

#[cfg(test)]
mod test {
    use super::*;
    test_expected_computed!(6, 1, test_config_d6());
    test_expected_computed!(6, 2, test_config_d6());
}
