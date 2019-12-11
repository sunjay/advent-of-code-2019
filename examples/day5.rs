use std::fs;

use anyhow::Error as AnyError;
use text_io::*;

fn main() -> Result<(), AnyError> {
    let program = fs::read_to_string("data/day5")?;

    let memory = program.split(',')
        .map(|s| s.trim().parse())
        .collect::<Result<Vec<i64>, _>>()?;

    evaluate(memory);

    Ok(())
}

pub fn evaluate(mut memory: Vec<i64>) {
    let mut pos = 0;
    loop {
        let instr = memory[pos];
        let param_modes = instr / 100;
        let opcode = instr - param_modes * 100;

        match opcode {
            1 => add(&mut memory, param_modes, &mut pos),
            2 => multiply(&mut memory, param_modes, &mut pos),
            3 => input(&mut memory, param_modes, &mut pos),
            4 => output(&mut memory, param_modes, &mut pos),
            99 => break,
            op => panic!("Encountered invalid opcode '{}' at position {}", op, pos),
        }
    }
}

/// Returns the value of the given parameter, either by returning its immediate value or by
/// dereferencing it if the parameter is in position mode
fn param_value(mem: &Vec<i64>, param_mode: i64, param_addr: usize) -> i64 {
    match param_mode {
        // position mode
        0 => mem[mem[param_addr] as usize],
        // immediate mode
        1 => mem[param_addr],
        // unknown mode
        _ => panic!("unknown parameter mode `{}`", param_mode),
    }
}

/// Returns the address at the address of the given target parameter.
///
/// Parameters that an instruction writes to will never be in immediate mode.
fn param_target(mem: &Vec<i64>, param_mode: i64, param_addr: usize) -> usize {
    assert_eq!(param_mode, 0, "expected parameter mode for param at position `{}` to be 0", param_addr);
    mem[param_addr] as usize
}

fn bin_op(
    mem: &mut Vec<i64>,
    param_modes: i64,
    pos: &mut usize,
    op: impl FnOnce(i64, i64) -> i64,
) {
    let left_addr = *pos + 1;
    let right_addr = *pos + 2;
    let target_addr = *pos + 3;
    *pos += 4;

    let target_mode = param_modes / 100;
    let right_mode = param_modes / 10 - target_mode * 10;
    let left_mode = param_modes / 1 - target_mode * 100 - right_mode * 10;

    let left = param_value(mem, left_mode, left_addr);
    let right = param_value(mem, right_mode, right_addr);
    let target = param_target(mem, target_mode, target_addr);

    mem[target] = op(left, right);
}

fn add(mem: &mut Vec<i64>, param_modes: i64, pos: &mut usize) {
    bin_op(mem, param_modes, pos, |x, y| x + y);
}

fn multiply(mem: &mut Vec<i64>, param_modes: i64, pos: &mut usize) {
    bin_op(mem, param_modes, pos, |x, y| x * y);
}

fn unary_op_read(
    mem: &mut Vec<i64>,
    param_modes: i64,
    pos: &mut usize,
    op: impl FnOnce(i64),
) {
    let target_addr = *pos + 1;
    *pos += 2;

    let param_mode = param_modes - param_modes / 10 * 10;
    let param_value = param_value(mem, param_mode, target_addr);

    op(param_value);
}

fn unary_op_write(
    mem: &mut Vec<i64>,
    param_modes: i64,
    pos: &mut usize,
    op: impl FnOnce(&mut i64),
) {
    let target_addr = *pos + 1;
    *pos += 2;

    let target_mode = param_modes - param_modes / 10 * 10;
    let target = param_target(mem, target_mode, target_addr);

    op(&mut mem[target]);
}

fn input(mem: &mut Vec<i64>, param_modes: i64, pos: &mut usize) {
    unary_op_write(mem, param_modes, pos, |m| *m = read!());
}

fn output(mem: &mut Vec<i64>, param_modes: i64, pos: &mut usize) {
    unary_op_read(mem, param_modes, pos, |m| println!("{}", m));
}
