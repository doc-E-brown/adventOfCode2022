use day3::{determine_badge, letter_to_score};
use std::{fs::File, io::Read};

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut buff = String::new();
    file.read_to_string(&mut buff).unwrap();
    let lines: Vec<&str> = buff.split_terminator("\n").collect::<Vec<&str>>();
    let groups: Vec<&[&str]> = lines.chunks_exact(3).collect();
    let score: u16 = groups
        .into_iter()
        .map(|x: &[&str]| -> u16 { letter_to_score(determine_badge(x).unwrap()).unwrap() })
        .sum();

    println!("Score: {}", score);
}
