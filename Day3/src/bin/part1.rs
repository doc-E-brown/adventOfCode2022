use day3::determine_score;
use std::{fs::File, io::Read};

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut buff = String::new();
    file.read_to_string(&mut buff).unwrap();
    let score: u16 = buff
        .split_terminator("\n")
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|x: &str| -> u16 { determine_score(x.to_string()).unwrap() })
        .sum();
    println!("Score: {}", score);
}
