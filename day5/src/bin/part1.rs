use std::{fs::File, io::Read};

fn main() {
    let mut file = File::open("initial_stack.txt").unwrap();
    let mut buff = String::new();
    file.read_to_string(&mut buff).unwrap();
    let stacks: Vec<&str> = buff.split_terminator("\n").collect::<Vec<&str>>();
    println!("{:#?}", stacks);

    let mut crane = day5::Crane::new(stacks, day5::CraneModel::CrateMover9000).unwrap();

    println!("Score: {:#?}", crane);

    let mut file = File::open("input.txt").unwrap();
    let mut buff = String::new();
    file.read_to_string(&mut buff).unwrap();
    let commands: Vec<&str> = buff.split_terminator("\n").collect::<Vec<&str>>();

    for cmd in commands.into_iter() {
        crane.execute_move(cmd).unwrap();
    }

    println!("Score: {:#?}", crane);
    println!("Score: {:#?}", crane.top_crates());
}
