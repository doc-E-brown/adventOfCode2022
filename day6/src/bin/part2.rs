use anyhow::Result;
use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::prelude::*;

fn main() -> Result<()> {
    let mut file = File::open("input.txt")?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    let raw_message = buffer.trim_end().chars().into_iter().collect::<Vec<char>>();

    let mut start_of_packet: VecDeque<char> = VecDeque::new();
    let mut pointer: usize = 0;
    let mut set: HashSet<char> = HashSet::new();

    for character in raw_message.iter() {
        // First character is in position 1
        pointer += 1;

        start_of_packet.push_back(*character);

        if start_of_packet.len() == 14 {
            for val in &start_of_packet {
                set.insert(*val);
            }

            if set.len() == 14 {
                break;
            }
            start_of_packet.pop_front();
            set.clear();
        }
    }

    println!("{:?}, {}", start_of_packet, pointer);

    Ok(())
}
