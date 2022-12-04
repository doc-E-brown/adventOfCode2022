use std::fs::File;
use std::io::Read;
fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut buff = String::new();
    file.read_to_string(&mut buff).unwrap();
    let elves: Vec<&str> = buff.split_terminator("\n\n").collect();
    let mut max_cal: u32 = 0;
    let mut max_elf: usize = 0;
    let mut top_cal: Vec<u32> = vec![0, 0, 0];
    let mut top_elf: Vec<usize> = vec![0, 0, 0];
    for (idx, elf) in elves.iter().enumerate() {
        let cals: u32 = elf
            .split_terminator("\n")
            .map(|x| -> u32 { x.parse::<u32>().unwrap() })
            .sum();
        // Check if in top 3
        for i in 0..3 {
            if cals > *top_cal.get(i).unwrap() {
                top_cal.insert(i, cals);
                top_elf.insert(i, idx);
                // Remove the last element
                top_cal.pop();
                top_elf.pop();
                break;
            };
        }
    }
    for (elf, cal) in top_elf.iter().zip(&top_cal) {
        println!("{:#?}, {:#?}", elf, cal);
    }
    let total_cal: u32 = top_cal.iter().sum();
    println!("{}", total_cal);
}
