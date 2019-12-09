use std::fs;

use anyhow::Error as AnyError;

fn main() -> Result<(), AnyError> {
    let program = fs::read_to_string("data/day2")?;

    let memory = program.split(',')
        .map(|s| s.trim().parse())
        .collect::<Result<Vec<usize>, _>>()?;

    println!("Part 1: {}", evaluate(memory.clone(), 12, 2));

    let target = 19690720;
    for x in 0..=99 {
        for y in 0..=99 {
            let value = evaluate(memory.clone(), x, y);
            if value == target {
                println!("Part 2: {}{}", x, y);
                break;
            }
        }
    }

    Ok(())
}

/// Takes the provided input values as the values at positions 1 and 2,
/// then returns the output produced at position 0 after evaluating the program.
pub fn evaluate(mut memory: Vec<usize>, input1: usize, input2: usize) -> usize {
    memory[1] = input1;
    memory[2] = input2;

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

    memory[0]
}

fn add(mem: &mut Vec<usize>, left: usize, right: usize, target: usize) {
    let target = mem[target];
    mem[target] = mem[mem[left]] + mem[mem[right]];
}

fn multiply(mem: &mut Vec<usize>, left: usize, right: usize, target: usize) {
    let target = mem[target];
    mem[target] = mem[mem[left]] * mem[mem[right]];
}
