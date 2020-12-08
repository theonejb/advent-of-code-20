use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead};
use crate::cpu::CPU;
use std::collections::HashSet;
use std::ops::Add;

mod cpu;

fn get_program(filename: &str) -> Vec<String> {
    let path = Path::new(filename);
    let file = File::open(path).unwrap();
    let lines = BufReader::new(file).lines();

    let mut program = vec![];
    for line in lines {
        program.push(line.unwrap());
    }

    program
}

fn does_program_loop(cpu: &mut CPU) -> bool {
    let mut visited_program_memory_locations = HashSet::new();

    while !cpu.program_finished() {
        if visited_program_memory_locations.contains(&cpu.get_instruction_pointer()) {
            return true;
        }

        visited_program_memory_locations.insert(cpu.get_instruction_pointer());
        cpu.next_cycle();
    }

    return false;
}

fn run_program_until_first_loop_or_exit(program: Vec<String>) -> bool {
    let mut cpu = CPU::new(program);

    let does_loop = does_program_loop(&mut cpu);
    if does_loop {
        println!("Value of accumulator right before loop: {}", cpu.get_accumulator());
    } else {
        println!("Value of accumulator right after exit: {}", cpu.get_accumulator());
    }

    !does_loop
}

fn try_to_fix_program(program: Vec<String>) {
    for (i, instruction) in program.iter().enumerate() {
        if instruction.starts_with("jmp") {
            println!("Changing jmp at {} to nop", i);
            let mut program = program.clone();
            program[i] = String::from("nop 0");

            if run_program_until_first_loop_or_exit(program) {
                break;
            }
        } else if instruction.starts_with("nop") {
            println!("Changing nop at {} to jmp", i);
            let mut program = program.clone();
            program[i] = String::from("nop ").add(&instruction[4..]);

            if run_program_until_first_loop_or_exit(program) {
                break;
            }
        }
    }
}

fn main() {
    let program = get_program("input.txt");
    try_to_fix_program(program);
}
