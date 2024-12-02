use std::{collections::HashMap, env, fs};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        return Err("No file specified".into());
    }

    let input = fs::read_to_string(&args[0])?;

    let mut left_list: Vec<usize> = Vec::new();
    let mut right_list: Vec<usize> = Vec::new();
    let mut right_counter_map: HashMap<usize, usize> = HashMap::new();

    for line in input.lines() {
        if let Some((left_number, right_number)) = line.split_once("   ") {
            left_list.push(left_number.parse()?);
            right_list.push(right_number.parse()?);
            right_counter_map
                .entry(right_number.parse()?)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }
    }

    left_list.sort_unstable();
    right_list.sort_unstable();

    let total_distance: usize = left_list
        .iter()
        .zip(right_list.iter())
        .map(|(l, r)| l.abs_diff(*r))
        .sum();

    let similarity_score: usize = left_list
        .iter()
        .map(|item| item * right_counter_map.get(item).unwrap_or(&0))
        .sum();

    println!("Result of puzzle 1: {total_distance}");
    println!("Result of puzzle 2: {similarity_score}");

    Ok(())
}
