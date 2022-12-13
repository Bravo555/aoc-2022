use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input/01.txt")?;

    let mut elves: Vec<i32> = input
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|line| line.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .collect();

    elves.sort_unstable();

    println!("{:?}", elves[elves.len() - 1]);

    println!(
        "{:?}",
        &elves[elves.len() - 3..elves.len()].iter().sum::<i32>()
    );

    Ok(())
}
