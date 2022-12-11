// Advent of Code Day SEVEN
//
#![allow(dead_code, non_camel_case_types, unused_imports)]
use aoc_22::*;
use id_tree::*;
use itertools::Itertools;
use std::collections::HashMap;
use std::{fs::File, io::Error, ops::Deref}; //import custom lib.rs module

const DAY: u8 = 0;
const TEST1_EXPECTED_OUTPUT: &str = "95437";
const TEST2_EXPECTED_OUTPUT: &str = "1";

#[derive(Debug, Clone)]
pub struct FileListing2 {
    name: String,
    size: usize,
    is_dir: bool,
    p_name: String,
    c_names: Vec<String>,
}
pub fn day_7_challenge_1(config: &Config) -> Result<i128, Error> {
    // Read and input data to a vector that is separated by words
    let input_string = read_into_string(config);
    let mut input_elements: Vec<Vec<&str>> = Vec::from(Vec::with_capacity(4));
    for line in input_string.lines() {
        let segmented_line: Vec<&str> = line.split_whitespace().collect();
        input_elements.push(segmented_line);
    }
    // parsing with a match statement
    //
    use id_tree::InsertBehavior::*;
    let mut sfs_tree: Tree<FileListing2> = TreeBuilder::new().with_node_capacity(1000).build();

    let root_id: NodeId = sfs_tree
        .insert(
            Node::new(FileListing2 {
                name: "/".to_string(),
                size: 0,
                is_dir: true,
                p_name: "".to_string(),
                c_names: Vec::new(),
            }),
            AsRoot,
        )
        .unwrap();
    #[allow(unused_variables, unused_mut)]
    let mut sfs_pointer = &root_id;

    fn match_to_tree<'a, 'b>(
        iter_line: Vec<&str>,
        tree: &'a mut Tree<FileListing2>,
        mut tree_pointer: &'a NodeId,
    ) -> &'a NodeId {
        //&dyn Iterator<Item = Vec<&str>>
        let mut term_line_iter = iter_line.into_iter().multipeek();
        let i1 = term_line_iter.next().unwrap_or(&"");
        let i2 = term_line_iter.next().unwrap_or(&"");
        let i3 = term_line_iter.next().unwrap_or(&"");
        // let sfs_tree_ref = &mut sfs_tree;
        // drop(sfs_tree_ref);
        match i1 {
            "$" => {
                match i2 {
                    "cd" => {
                        match i3 {
                            ".." => {
                                if let Some(_) = Some(0) {
                                    tree_pointer =
                                        tree.get(&tree_pointer).unwrap().parent().unwrap();
                                }
                            }
                            "/" => {
                                "/".to_string();
                                tree_pointer = tree.root_node_id().unwrap();
                            }
                            _ => {
                                i3.to_string();
                                for child in tree.children_ids(&tree_pointer).unwrap() {
                                    if tree.get(child).unwrap().data().name == i3 {
                                        tree_pointer = child;
                                    }
                                }
                                // current_pointer = current_pointer.
                            }
                        };
                    }
                    "ls" => {}
                    _ => {}
                };
            }
            "dir" => {
                #[allow(unused_variables)]
                let new_node = tree
                    .insert(
                        Node::new(FileListing2 {
                            name: i2.to_string(),
                            size: 0,
                            is_dir: true,
                            p_name: "".to_string(),
                            c_names: Vec::new(),
                        }),
                        UnderNode(&tree_pointer),
                    )
                    .unwrap();
            }
            // "123" => {  },
            _ => {
                // if i2.map(|| true) {
                if i1.parse::<usize>().is_ok() {}
            }
        };

        tree_pointer
    }

    #[allow(unused_mut)]
    let mut iter_term = input_elements.into_iter();
    #[allow(unused_variables, unreachable_code)]
    for term_line in iter_term {
        // NOTE: WORK IN PROGRESS
        // let sfs_pointer_temp = match_to_tree(term_line, &mut sfs_tree, sfs_pointer);
        // sfs_pointer = sfs_pointer_temp;
        // NOTE: Remove below if function works1
        // let mut term_line_iter = term_line.into_iter().multipeek();
        // let i1 = term_line_iter.next().unwrap_or(&"").trim();
        // let i2 = term_line_iter.next().unwrap_or(&"").trim();
        // let i3 = term_line_iter.next().unwrap_or(&"").trim();
        // // let sfs_tree_ref = &mut sfs_tree;
        // // drop(sfs_tree_ref);
        // match i1 {
        //     "$" => {
        //         match i2 {
        //             "cd" => {
        //                 match i3 {
        //                     ".." => {
        //                         if let Some(_) = Some(0) {
        //                              sfs_pointer = sfs_tree.get(sfs_pointer).unwrap().parent().unwrap();
        //                         }
        //                     }
        //                     "/" => {
        //                          "/".to_string();
        //                          sfs_pointer = &root_id;
        //                     }
        //                     _ => {
        //                         i3.to_string();
        //                         for child in sfs_tree.children_ids(sfs_pointer).unwrap() {
        //                             if sfs_tree.get(child).unwrap().data().name == i3 {
        //                                 sfs_pointer = child;
        //                             }
        //                         }
        //                         // current_pointer = current_pointer.
        //                     }
        //                 };
        //             }
        //             "ls" => {}
        //             _ => {}
        //         };
        //     }
        //     "dir" => {
        //      // let new_node = sfs_tree.insert(Node::new(FileListing2 { name: i2.to_string(), size: 0, is_dir: true, p_name: "".to_string(), c_names: Vec::new() }) , UnderNode(sfs_pointer)).unwrap();

        //     }
        //     // "123" => {  },
        //     _ => {
        //         // if i2.map(|| true) {
        //         if i1.parse::<usize>().is_ok() {

        //         }
        //     }
        // };
    }
    let mut size_over_10k: usize = 0;
    let size = 1;
    if size <= 100_000 {
        size_over_10k += size;
    }

    fn add_sizes(entry: FileListing2, fs: &HashMap<String, FileListing2>) -> usize {
        let mut sum_sizes: usize = 0;
        if entry.size == 0 {
            for child in entry.c_names.into_iter() {
                sum_sizes += add_sizes(fs.get(&child).unwrap().to_owned(), &fs);
            }
        } else {
            sum_sizes += entry.size;
        }
        sum_sizes
    }

    Ok(size_over_10k as i128)
}

pub struct FileListing<'a> {
    name: &'a str,
    size: usize,
    is_dir: bool,
}

enum Command {
    ls,
    dir,
    cd(String),
}
enum Output {
    DirList(Vec<(String, usize)>),
    None,
}
enum Terminal {
    Command(Command),
    Output(Output),
}

#[allow(unused_variables)]
pub fn day_7_challenge_2(config: &Config) -> Result<i128, Error> {
    Ok(0)
}

pub fn test_config_d7() -> Config {
    let test_config: Config = Config {
        challenge: 7,
        filename: "./input/test7.txt".to_string(),
    };
    test_config
}

#[cfg(test)]
mod test {
    use super::*;
    test_expected_computed!(7, 1, test_config_d7());
    test_expected_computed!(7, 2, test_config_d7());
}
