use crate::{DecoderChip, Instruction};
use std::collections::HashSet;
use std::iter::FromIterator;

#[test]
fn test_compile_instruction() {
    let set_mask = DecoderChip::compile_instruction("mask = 00110X11X0000110X0000001000111010X00");
    assert_eq!(set_mask, Instruction::SetMask(String::from("00110X11X0000110X0000001000111010X00")));

    let set_memory = DecoderChip::compile_instruction("mem[61385] = 13441");
    assert_eq!(set_memory, Instruction::SetMemory(61385, 13441));
}

#[test]
fn test_compile_mask() {
    let mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X";
    let compiled_mask = DecoderChip::compile_mask(mask);

    assert_eq!(compiled_mask.or_value, 0b1000000, "Or mask");
    assert_eq!(compiled_mask.and_value,
               0b1111111111111111111111111111111111111111111111111111111111111101,
               "And mask");

    let compiled_mask = DecoderChip::compile_mask("000000000000000000000000000000X1001X");
    assert_eq!(compiled_mask.floating_indices, vec![0, 5]);
}

#[test]
fn test_apply_mask_v2() {
    let mask = DecoderChip::compile_mask("000000000000000000000000000000X1001X");
    let possible_values = mask.apply_maks_v2(42);
    let expected_values = vec![26, 27, 58, 59];

    let mut values_set = HashSet::new();
    for v in possible_values {
        values_set.insert(v);
    }

    for v in expected_values.iter() {
        assert!(values_set.contains(v));
    }

    let mask = DecoderChip::compile_mask("00000000000000000000000000000000X0XX");
    let possible_values = mask.apply_maks_v2(26);
    let expected_values = vec![16, 17, 18, 19, 24, 25, 26, 27];

    let mut values_set = HashSet::new();
    for v in possible_values {
        values_set.insert(v);
    }

    for v in expected_values.iter() {
        assert!(values_set.contains(v));
    }
}

#[test]
fn test_apply_mask() {
    let mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X";
    let mask = DecoderChip::compile_mask(mask);

    assert_eq!(mask.apply_mask(11), 73);
    assert_eq!(mask.apply_mask(101), 101);
    assert_eq!(mask.apply_mask(0), 64);
}

#[test]
fn test_compile_and_apply_instruction() {
    let mut chip = DecoderChip::new();

    chip.compile_and_apply_instruction("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X");

    chip.compile_and_apply_instruction("mem[8] = 11");
    assert_eq!(*chip.set_memory_locations.get(&8).unwrap(), 73);

    chip.compile_and_apply_instruction("mem[7] = 101");
    assert_eq!(*chip.set_memory_locations.get(&7).unwrap(), 101);

    chip.compile_and_apply_instruction("mem[8] = 0");
    assert_eq!(*chip.set_memory_locations.get(&8).unwrap(), 64);
}

#[test]
fn test_decoder_chip() {
    let program = vec![
        String::from("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"),
        String::from("mem[8] = 11"),
        String::from("mem[7] = 101"),
        String::from("mem[8] = 0"),
    ];
    let mut chip = DecoderChip::new();
    chip.run_program(&program);
    assert_eq!(chip.sum_all_memory_values(), 165);
}