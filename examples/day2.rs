use std::fs;

use anyhow::Error as AnyError;

fn main() -> Result<(), AnyError> {
    let program = fs::read_to_string("data/day2")?;

    let mut memory = program.split(',')
        .map(|s| s.trim().parse())
        .collect::<Result<Vec<usize>, _>>()?;

    let mut pos = 0;
    loop {
        match memory[pos] {
            1 => add(&mut memory, pos+1, pos+2, pos+3),
            2 => multiply(&mut memory, pos+1, pos+2, pos+3),
            99 => break,
            op => panic!("Encountered invalid opcode '{}' at position {}", op, pos),
        }

        pos += 4;
    }

    println!("{}", memory[0]);

    Ok(())
}

fn add(mem: &mut Vec<usize>, left: usize, right: usize, target: usize) {
    let target = mem[target];
    mem[target] = mem[mem[left]] + mem[mem[right]];
}

fn multiply(mem: &mut Vec<usize>, left: usize, right: usize, target: usize) {
    let target = mem[target];
    mem[target] = mem[mem[left]] * mem[mem[right]];
}
