use regex::Regex;

mod tests;

#[derive(Debug)]
pub struct CPU {
    accumulator: i64,
    instruction_pointer: usize,

    program_memory: Vec<String>,
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Nop,
    Acc(i32),
    Jmp(i32),
}

impl CPU {
    pub fn new(program: Vec<String>) -> CPU {
        CPU {
            accumulator: 0,
            instruction_pointer: 0,
            program_memory: program,
        }
    }

    pub fn get_accumulator(&self) -> i64 {
        self.accumulator
    }

    pub fn get_instruction_pointer(&self) -> usize {
        self.instruction_pointer
    }

    pub fn next_cycle(&mut self) {
        let instruction = &self.program_memory[self.instruction_pointer];

        match CPU::decode_instruction(&instruction[..]) {
            Instruction::Nop => { self.do_nop() }
            Instruction::Acc(arg) => { self.do_acc(arg) }
            Instruction::Jmp(arg) => { self.do_jmp(arg) }
        }
    }

    pub fn program_finished(&self) -> bool {
        self.instruction_pointer >= self.program_memory.len()
    }

    fn do_nop(&mut self) {
        self.instruction_pointer += 1;
    }

    fn do_acc(&mut self, arg: i32) {
        self.accumulator += arg as i64;
        self.instruction_pointer += 1;
    }

    fn do_jmp(&mut self, arg: i32) {
        self.instruction_pointer = (self.instruction_pointer as isize + arg as isize) as usize;
    }

    fn decode_instruction(instruction: &str) -> Instruction {
        let re = Regex::new(r"^(\w+)\s([-+]?\d+)$").unwrap();
        let cap = re.captures(instruction);
        if let None = cap {
            return Instruction::Nop;
        }

        let cap = cap.unwrap();
        let op = cap.get(1).unwrap().as_str();
        let arg = cap.get(2).unwrap().as_str();
        let arg = arg.parse::<i32>().unwrap();

        match op {
            "acc" => Instruction::Acc(arg),
            "jmp" => Instruction::Jmp(arg),
            _ => Instruction::Nop
        }
    }
}