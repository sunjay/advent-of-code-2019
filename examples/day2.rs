use std::fs;

use anyhow::Error as AnyError;

fn main() -> Result<(), AnyError> {
    let program = fs::read_to_string("data/day2")?;

    let memory = program.split(',')
        .map(|s| s.trim().parse())
        .collect::<Result<Vec<usize>, _>>()?;

    let prog = Program::new(memory);
    println!("Part 1: {}", prog.evaluate(12, 2));

    Ok(())
}

#[derive(Debug)]
pub struct Address(pub usize);

#[derive(Debug)]
enum Instruction {
    Add {left: Address, right: Address, target: Address},
    Mul {left: Address, right: Address, target: Address},
    Halt,
}

impl Instruction {
    pub fn parse(memory: &[usize], pos: &mut usize) -> Self {
        use Instruction::*;

        let bin_op = |pos: &mut usize| {
            let left = Address(memory[*pos+1]);
            let right = Address(memory[*pos+2]);
            let target = Address(memory[*pos+3]);

            *pos += 4;

            (left, right, target)
        };

        match memory[*pos] {
            1 => {
                let (left, right, target) = bin_op(pos);
                Add {left, right, target}
            },
            2 => {
                let (left, right, target) = bin_op(pos);
                Mul {left, right, target}
            },
            99 => {
                *pos += 1;
                Halt
            },
            op => panic!("Encountered invalid opcode '{}' at position {}", op, pos),
        }
    }

    pub fn evaluate(&self, memory: &mut [usize]) ->  {

    }
}

#[derive(Debug, Clone)]
struct Program {
    memory: Vec<usize>,
}

impl Program {
    pub fn new(memory: Vec<usize>) -> Self {
        Self {
            memory,
        }
    }

    /// Takes the provided input values as the values at positions 1 and 2,
    /// then returns the output produced at position 0 after evaluating the program.
    pub fn evaluate(mut self, input1: usize, input2: usize) -> usize {
        let mut memory = &mut self.memory;
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
}

fn add(mem: &mut Vec<usize>, left: usize, right: usize, target: usize) {
    let target = mem[target];
    mem[target] = mem[mem[left]] + mem[mem[right]];
}

fn multiply(mem: &mut Vec<usize>, left: usize, right: usize, target: usize) {
    let target = mem[target];
    mem[target] = mem[mem[left]] * mem[mem[right]];
}
