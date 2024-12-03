use std::{env, fs};

use regex::Regex;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        return Err("No file specified".into());
    }

    let input = fs::read_to_string(&args[0])?;

    let mut result = 0;
    let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();

    for line in input.lines() {
        let matches: Vec<_> = re.find_iter(line).map(|m| m.as_str()).collect();

        for m in matches {
            if let Some((a, b)) = &m[4..(m.len() - 1)].split_once(',') {
                let a: usize = a.parse()?;
                let b: usize = b.parse()?;

                result += a * b;
            }
        }
    }

    println!("Result of puzzle 1: {result}");

    Ok(())
}
