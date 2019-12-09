use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::Error as AnyError;

fn main() -> Result<(), AnyError> {
    let input_file = File::open("data/day1")?;
    let input_reader = BufReader::new(input_file);

    let mut total_fuel = 0;
    for mass_str in input_reader.lines() {
        let mass = mass_str?.parse()?;
        total_fuel += fuel_required(mass);
    }

    println!("Total fuel required: {}", total_fuel);

    Ok(())
}

/// Returns the amount of fuel needed by a module based on its mass
fn fuel_required(mass: i64) -> i64 {
    let fuel = mass / 3 - 2;
    if fuel <= 0 {
        return 0;
    }

    let fuel_for_fuel = fuel_required(fuel);
    fuel + fuel_for_fuel
}
