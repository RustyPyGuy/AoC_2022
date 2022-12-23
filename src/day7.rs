// Advent of Code Day SEVEN
//
#![allow(dead_code, non_camel_case_types, unused_imports)]
use aoc_22::*;
use id_tree::*;
use itertools::Itertools;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::{
    fs::File,
    io::Error,
    ops::{Deref, DerefMut},
}; //import custom lib.rs module

const DAY: u8 = 0;
const TEST1_EXPECTED_OUTPUT: &str = "95437";
const TEST2_EXPECTED_OUTPUT: &str = "24933642";

#[derive(Debug, Clone)]
pub struct FileListing2 {
    name: String,
    size: usize,
    is_dir: bool,
}
pub fn day_7_challenge_1(config: &Config) -> Result<i128, Error> {
    // Read and input data to a vector that is separated by words
    let input_string = read_into_string(config);
    let mut input_elements: Vec<Vec<&str>> = Vec::from(Vec::with_capacity(4));
    for line in input_string.lines() {
        let segmented_line: Vec<&str> = line.split_whitespace().collect();
        input_elements.push(segmented_line);
    }
    let mut sfs_tree: Tree<FileListing2> = TreeBuilder::new().with_node_capacity(1000).build();
    let root_id: NodeId = sfs_tree
        .insert(
            Node::new(FileListing2 {
                name: "/".to_string(),
                size: 0,
                is_dir: true,
            }),
            InsertBehavior::AsRoot,
        )
        .unwrap();
    let sfs_pointer = root_id.clone();
    let mut tree_refcell: Rc<RefCell<NodeId>> = Rc::new(RefCell::new(sfs_pointer));

    let iter_term = input_elements.into_iter();
    for term_line in iter_term {
        create_tree_node_from_line_match(term_line, &mut sfs_tree, &mut tree_refcell);
    }
    apply_dir_sizes(&mut sfs_tree);
    let mut tree_traversal = sfs_tree.traverse_level_order_ids(&root_id).unwrap();
    let mut size_over_10k: usize = 0;
    while let Some(item_id) = tree_traversal.next() {
        if sfs_tree.get(&item_id).unwrap().data().is_dir == true {
            let dir_recurse_size = pseudo_recursive_add_sizes(&item_id, &sfs_tree);
            if dir_recurse_size <= 100_000 {
                size_over_10k += dir_recurse_size;
            }
        }
    }

    // ENABLE below for a pretty tree print-out.
    // let mut s = String::new();
    // sfs_tree.write_formatted(&mut s).unwrap();
    // print!("{}", s);

    Ok(size_over_10k as i128)
}

pub fn day_7_challenge_2(config: &Config) -> Result<i128, Error> {
    // Read and input data to a vector that is separated by words
    let input_string = read_into_string(config);
    let mut input_elements: Vec<Vec<&str>> = Vec::from(Vec::with_capacity(4));
    for line in input_string.lines() {
        let segmented_line: Vec<&str> = line.split_whitespace().collect();
        input_elements.push(segmented_line);
    }
    let mut sfs_tree: Tree<FileListing2> = TreeBuilder::new().with_node_capacity(1000).build();

    let root_id: NodeId = sfs_tree
        .insert(
            Node::new(FileListing2 {
                name: "/".to_string(),
                size: 0,
                is_dir: true,
            }),
            InsertBehavior::AsRoot,
        )
        .unwrap();
    let sfs_pointer = root_id.clone();
    let mut tree_refcell: Rc<RefCell<NodeId>> = Rc::new(RefCell::new(sfs_pointer));

    let iter_term = input_elements.into_iter();
    for term_line in iter_term {
        create_tree_node_from_line_match(term_line, &mut sfs_tree, &mut tree_refcell);
    }
    apply_dir_sizes(&mut sfs_tree);
    let total_occupied_space = pseudo_recursive_add_sizes(&root_id, &sfs_tree);
    let minimum_delete_amount = 30000000 - (70000000 - total_occupied_space);
    let mut delete_amount_candidate: usize = usize::MAX;
    let mut tree_traversal = sfs_tree.traverse_level_order_ids(&root_id).unwrap();
    while let Some(item_id) = tree_traversal.next() {
        if sfs_tree.get(&item_id).unwrap().data().is_dir == true {
            let dir_recurse_size = pseudo_recursive_add_sizes(&item_id, &sfs_tree);
            if dir_recurse_size >= minimum_delete_amount
                && dir_recurse_size < delete_amount_candidate
            {
                delete_amount_candidate = dir_recurse_size;
            }
        }
    }
    // ENABLE below for a pretty tree print-out.
    // let mut s = String::new();
    // sfs_tree.write_formatted(&mut s).unwrap();
    // print!("{}", s);
    Ok(delete_amount_candidate as i128)
}

// takes one line of input and creates a node or assigns the pointer to a new directory.  This
// function should be claled in loop iterations.  Modifies borrowed variables.
// The pointer is a refcell which is necessary because mutable and immutable borrows of the tree
// occur at the same time.
fn create_tree_node_from_line_match<'a, 'b>(
    iter_line: Vec<&str>,
    tree: &'a mut Tree<FileListing2>,
    tree_refcell: &'a mut Rc<RefCell<NodeId>>,
) {
    let mut term_line_iter = iter_line.into_iter().multipeek();
    let i1 = term_line_iter.next().unwrap_or(&"");
    let i2 = term_line_iter.next().unwrap_or(&"");
    let i3 = term_line_iter.next().unwrap_or(&"");
    match i1 {
        "$" => {
            match i2 {
                "cd" => {
                    match i3 {
                        ".." => {
                            // if let Some(_) = Some(0) {
                            // NOTE: This unwrap_or is a guess to prevent panic

                            let parent_id = tree.get( tree_refcell.borrow().deref()).unwrap().parent().unwrap_or_else(|| {println!("no value for parent. current node: {:?} moving to root node.", tree_refcell.borrow().deref());tree.root_node_id().unwrap()});
                            *tree_refcell.borrow_mut() = parent_id.clone();
                        }
                        "/" => {
                            "/".to_string();
                            *tree_refcell.borrow_mut() = tree.root_node_id().unwrap().clone();
                        }
                        _ => {
                            let mut temp_pointer: NodeId = tree_refcell.borrow().deref().clone();
                            i3.to_string();
                            for child in tree.children_ids(tree_refcell.borrow().deref()).unwrap() {
                                if tree.get(child).unwrap().data().name == i3 {
                                    temp_pointer = child.clone();
                                }
                            }
                            *tree_refcell.borrow_mut() = temp_pointer;
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
                    }),
                    InsertBehavior::UnderNode(tree_refcell.borrow().deref()),
                )
                .unwrap();
        }
        _ => {
            // if i2.map(|| true) {
            if i1.parse::<usize>().is_ok() {
                #[allow(unused_variables)]
                let new_node = tree
                    .insert(
                        Node::new(FileListing2 {
                            name: i2.to_string(),
                            size: i1.parse::<usize>().unwrap(),
                            is_dir: false,
                        }),
                        InsertBehavior::UnderNode(tree_refcell.borrow().deref()),
                    )
                    .unwrap();
            } else {
                println!("decoding error in file listing");
            }
        }
    };
}

fn apply_dir_sizes<'a, 'b>(tree: &'a mut Tree<FileListing2>) {
    let mut dir_size_vec: Vec<(NodeId, usize, FileListing2)> = Vec::new();
    let mut tree_traversal = tree
        .traverse_level_order_ids(&tree.root_node_id().unwrap().clone())
        .unwrap();
    while let Some(item_id) = tree_traversal.next() {
        let temp_item = tree.get(&item_id).unwrap().data();
        if temp_item.is_dir == true {
            let mut local_total: usize = 0;
            for child in tree.get(&item_id).unwrap().children().into_iter() {
                if tree.get(child).unwrap().data().is_dir == false {
                    local_total += tree.get(child).unwrap().data().size;
                }
            }
            dir_size_vec.push((item_id.clone(), local_total, temp_item.clone()));
        }
    }
    // separate loop defined to take the borrows out of scope from the previous loop.
    for (node_id, size, mut file_listing) in dir_size_vec.into_iter() {
        file_listing.size = size;
        tree.get_mut(&node_id).unwrap().replace_data(file_listing);
    }
}

// add sizes of all directories and files under the specified directory node.
fn pseudo_recursive_add_sizes(node_id: &NodeId, tree: &Tree<FileListing2>) -> usize {
    let mut cumulative_size: usize = 0;
    let mut tree_traversal = tree.traverse_level_order(&node_id).unwrap();
    while let Some(item) = tree_traversal.next() {
        if item.data().is_dir == true {
            cumulative_size += item.data().size;
        }
    }
    cumulative_size
}

// unit test config function
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
