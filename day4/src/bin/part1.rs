use std::{fs::File, io::Read};

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut buff = String::new();
    file.read_to_string(&mut buff).unwrap();
    let assignments: Vec<Vec<&str>> = buff
        .split_terminator("\n")
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|x: &str| -> Vec<&str> { x.split_terminator(",").collect() })
        .collect();

    let overlap: u16 = assignments
        .into_iter()
        .map(|x| -> u16 { day4::complete_overlap(x.as_slice()).unwrap() })
        .sum();

    println!("Score: {:#?}", overlap);
}
