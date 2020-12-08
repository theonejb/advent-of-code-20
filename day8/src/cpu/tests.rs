use super::*;

#[test]
fn test_decode_instruction() {
    assert_eq!(CPU::decode_instruction("nop"), Instruction::Nop);

    assert_eq!(CPU::decode_instruction("acc +0"), Instruction::Acc(0));
    assert_eq!(CPU::decode_instruction("acc -99"), Instruction::Acc(-99));

    assert_eq!(CPU::decode_instruction("jmp -99"), Instruction::Jmp(-99));
}

#[test]
fn test_nop() {
    let mut cpu = CPU::new(vec![]);
    cpu.do_nop();

    assert_eq!(cpu.instruction_pointer, 1);
    assert_eq!(cpu.accumulator, 0);
}

#[test]
fn test_acc() {
    let mut cpu = CPU::new(vec![]);

    cpu.do_acc(10);
    assert_eq!(cpu.instruction_pointer, 1);
    assert_eq!(cpu.accumulator, 10);

    cpu.do_acc(-20);
    assert_eq!(cpu.instruction_pointer, 2);
    assert_eq!(cpu.accumulator, -10);
}

#[test]
fn test_jmp() {
    let mut cpu = CPU::new(vec![]);

    cpu.do_jmp(10);
    assert_eq!(cpu.instruction_pointer, 10);

    cpu.do_jmp(-5);
    assert_eq!(cpu.instruction_pointer, 5);
}