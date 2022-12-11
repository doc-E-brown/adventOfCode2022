use std::fs::File;
use std::io::prelude::*;

use day8::{find_max_scenic_score, Trees};

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();

    let trees = Trees::from_string(&buffer);

    // Part 1
    let visibility_map = trees.get_visibility().unwrap();
    println!("Visible trees: {}", visibility_map.sum_visible_trees());

    // Part 2
    let scenic_scores = trees.compute_all_scenic_score().unwrap();
    let max_score = find_max_scenic_score(&scenic_scores.grid).unwrap();

    println!("Most scenic tree: {}", max_score);
}
