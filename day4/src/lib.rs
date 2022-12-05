pub(crate) use anyhow::{bail, Result};
fn min_max_assignment(assignment: &str) -> Result<(u16, u16)> {
    let min_max: Vec<u16> = assignment
        .split_terminator("-")
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|x: &str| -> u16 { x.parse::<u16>().unwrap() })
        .collect();
    if min_max.len() != 2 {
        bail!("Invalid assignment specification")
    }

    Ok((min_max[0], min_max[1]))
}

pub fn complete_overlap(assignments: &[&str]) -> Result<u16> {
    let (elf1_min, elf1_max) = min_max_assignment(assignments[0])?;
    let (elf2_min, elf2_max) = min_max_assignment(assignments[1])?;

    let result = (elf1_max <= elf2_max) && (elf1_min >= elf2_min)
        || (elf2_max <= elf1_max) && (elf2_min >= elf1_min);

    println!(
        "Elf1: ({}, {})\nElf2: ({}, {})\n Result: {}\n",
        elf1_min, elf1_max, elf2_min, elf2_max, result
    );

    Ok(result as u16)
}

pub fn any_overlap(assignments: &[&str]) -> Result<u16> {
    let (elf1_min, elf1_max) = min_max_assignment(assignments[0])?;
    let (elf2_min, elf2_max) = min_max_assignment(assignments[1])?;

    let result = (elf1_max >= elf2_min) && (elf1_min <= elf2_max)
        || (elf2_max >= elf1_min) && (elf2_min <= elf1_max);

    println!(
        "Elf1: ({}, {})\nElf2: ({}, {})\n Result: {}\n",
        elf1_min, elf1_max, elf2_min, elf2_max, result
    );

    Ok(result as u16)
}
