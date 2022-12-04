pub(crate) use anyhow::{bail, Result};

pub fn letter_to_score(some_code: char) -> Result<u16> {
    let val: u16 = some_code as u16;
    if some_code.is_numeric() {
        bail!("Invalid value")
    }
    if some_code.is_uppercase() {
        Ok(27 + (val - 'A' as u16))
    } else {
        Ok((val + 1) - 'a' as u16)
    }
}

pub fn determine_score(some_code: String) -> Result<u16> {
    let (comp1, comp2) = some_code.split_at(some_code.len() / 2);
    //  Contains exactly one overlap

    let mut overlap: Vec<char> = comp1
        .chars()
        .filter(|x| -> bool { comp2.contains(*x) })
        .collect();
    overlap.dedup();

    if overlap.len() != 1 {
        bail!("Only one character overlap allowed {:#?}", overlap);
    }
    println!(
        "{},{}, {:?}, {:?}",
        comp1,
        comp2,
        overlap,
        letter_to_score(overlap[0]).unwrap()
    );
    Ok(letter_to_score(overlap[0]).unwrap())
}

pub fn determine_badge(rucksacks: &[&str]) -> Result<char> {
    let mut overlap: Vec<char> = rucksacks[0]
        .chars()
        .filter(|x| -> bool { rucksacks[1].contains(*x) && rucksacks[2].contains(*x) })
        .collect();
    overlap.dedup();

    if overlap.len() != 1 {
        bail!("Only one character overlap allowed {:#?}", overlap);
    }

    Ok(overlap[0])
}
