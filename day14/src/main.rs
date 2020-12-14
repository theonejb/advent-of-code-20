mod tests;

use std::collections::HashMap;
use regex;
use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead};

/*
We only keep the latest mask, and only store those memory locations which have been set to non-zero.
This let's us not have to allocate ~68GB of memory for it. Seems like a good thing.
 */
struct DecoderChip {
    mask: CompiledMask,
    set_memory_locations: HashMap<u64, u64>,
}

/*
We compile the mask string into 2 values, an "or value" and a "and value".

The "or value" starts with all 0s.
For bits that need to be set to 1, we set those bits in the "or value" to 1.

The "and value" starts with all 1s.
For bits that need to be set to 0, we set those bits in the "and value" to 0.

To apply, we use the & and | operators with the corresponding values on the incoming data.
 */
#[derive(PartialEq, Debug)]
struct CompiledMask {
    or_value: u64,
    and_value: u64,
    floating_indices: Vec<u8>,
}

impl CompiledMask {
    pub fn apply_mask(&self, to: u64) -> u64 {
        (to | self.or_value) & self.and_value
    }

    fn apply_floating_mask(indices: &[u8], to: u64) -> Vec<u64> {
        if indices.is_empty() {
            return vec![to];
        }

        let index_to_work_on = indices[0];
        let zero_mask = 0xFFFFFFFFFFFFFFFFu64 ^ (1 << index_to_work_on);
        let one_mask = 0u64 | (1 << index_to_work_on);

        let zero_value = to & zero_mask;
        let one_value = to | one_mask;

        let mut options_with_zero_value = CompiledMask::apply_floating_mask(&indices[1..], zero_value);
        let mut options_with_one_value = CompiledMask::apply_floating_mask(&indices[1..], one_value);

        let mut all_options = options_with_zero_value;
        all_options.append(&mut options_with_one_value);

        all_options
    }

    pub fn apply_mask_v2(&self, to: u64) -> Vec<u64> {
        CompiledMask::apply_floating_mask(&self.floating_indices, to | self.or_value)
    }
}

#[derive(PartialEq, Debug)]
enum Instruction {
    SetMask(String),
    SetMemory(u64, u64),
}

impl DecoderChip {
    pub fn new() -> DecoderChip {
        DecoderChip {
            mask: CompiledMask { or_value: 0, and_value: 0, floating_indices: vec![] },
            set_memory_locations: HashMap::new(),
        }
    }
    pub fn compile_instruction(instruction: &str) -> Instruction {
        let mut instruction_parts = instruction.split(" = ");
        let operator = instruction_parts.next();
        let operand = instruction_parts.next();

        let operator = operator.unwrap();
        let operand = operand.unwrap();

        if operator == "mask" {
            return Instruction::SetMask(String::from(operand));
        }

        let operand = operand.parse::<u64>().unwrap();
        let re = regex::Regex::new(r"^mem\[(\d+)\]$").unwrap();
        let memory_address = re.captures(operator).unwrap().get(1).unwrap().as_str();
        let memory_address = memory_address.parse::<u64>().unwrap();

        return Instruction::SetMemory(memory_address, operand);
    }

    pub fn compile_mask(mask: &str) -> CompiledMask {
        let (mut or_value, mut and_value) = (0u64, 0xFFFFFFFFFFFFFFFFu64);
        let mut floating_indices = vec![];

        for (i, mask_value) in mask.chars().rev().enumerate() {
            match mask_value {
                '1' => {
                    or_value |= 1 << i;
                }
                '0' => {
                    and_value ^= 1 << i;
                }
                'X' => {
                    floating_indices.push(i as u8);
                }
                _ => ()
            }
        }

        CompiledMask { or_value, and_value, floating_indices }
    }

    fn apply_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::SetMask(mask) => {
                self.mask = DecoderChip::compile_mask(&mask[..]);
            }
            Instruction::SetMemory(at, value) => {
                self.set_memory_locations.insert(at, self.mask.apply_mask(value));
            }
        }
    }

    fn apply_instruction_v2(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::SetMask(mask) => {
                self.mask = DecoderChip::compile_mask(&mask[..]);
            }
            Instruction::SetMemory(at, value) => {
                for memory_locations in self.mask.apply_mask_v2(at) {
                    self.set_memory_locations.insert(memory_locations, value);
                }
            }
        }
    }

    pub fn compile_and_apply_instruction(&mut self, instruction: &str) {
        let instruction = DecoderChip::compile_instruction(instruction);
        self.apply_instruction(instruction);
    }

    pub fn compile_and_apply_instruction_v2(&mut self, instruction: &str) {
        let instruction = DecoderChip::compile_instruction(instruction);
        self.apply_instruction_v2(instruction);
    }

    pub fn run_program(&mut self, program: &Vec<String>) {
        for instruction in program.iter() {
            self.compile_and_apply_instruction(&instruction[..]);
        }
    }

    pub fn run_program_v2(&mut self, program: &Vec<String>) {
        for instruction in program.iter() {
            self.compile_and_apply_instruction_v2(&instruction[..]);
        }
    }

    pub fn sum_all_memory_values(&self) -> u64 {
        let mut sum = 0;
        for (_, value) in self.set_memory_locations.iter() {
            sum += value;
        }

        sum
    }
}

fn get_input(filename: &str) -> Vec<String> {
    let p = Path::new(filename);
    let f = File::open(p).unwrap();
    let lines = BufReader::new(f).lines();

    let mut input = vec![];
    for line in lines {
        input.push(line.unwrap());
    }

    input
}

fn main() {
    let input = get_input("input.txt");

    let mut chip = DecoderChip::new();
    chip.run_program(&input);
    println!("Result part 1: {}", chip.sum_all_memory_values());

    let mut chip_v2 = DecoderChip::new();
    chip_v2.run_program_v2(&input);
    println!("Result part 2: {}", chip_v2.sum_all_memory_values());
}
