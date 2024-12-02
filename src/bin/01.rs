use std::{env, fs};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        return Err("No file specified".into());
    }

    let input = fs::read_to_string(&args[0])?;

    let mut left_list: Vec<usize> = Vec::new();
    let mut right_list: Vec<usize> = Vec::new();

    for line in input.lines() {
        if let Some((left_number, right_number)) = line.split_once("   ") {
            left_list.push(left_number.parse()?);
            right_list.push(right_number.parse()?);
        }
    }

    left_list.sort_unstable();
    right_list.sort_unstable();

    let total_distance: usize = left_list
        .iter()
        .zip(right_list.iter())
        .map(|(l, r)| l.abs_diff(*r))
        .sum();

    println!("Result of puzzle 1: {total_distance}");

    Ok(())
}
