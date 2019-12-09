use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::Error as AnyError;

fn main() -> Result<(), AnyError> {
    let input_file = File::open("data/day1")?;
    let input_reader = BufReader::new(input_file);

    let mut total_mass = 0;
    for mass_str in input_reader.lines() {
        let mass = mass_str?.parse()?;
        total_mass += module_fuel(mass);
    }

    println!("Total fuel required: {}", total_mass);

    Ok(())
}

/// Returns the amount of fuel needed by a module based on its mass
fn module_fuel(mass: u64) -> u64 {
    mass / 3 - 2
}
