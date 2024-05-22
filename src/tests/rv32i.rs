use crate::Decoder;
use std::fs::read_to_string;

#[test]
fn test_rv32i() {
    let mut test_decoder =
        Decoder::new(&vec![include_str!("../../examples/RV32I.toml").to_string()]);

    read_to_string("src/tests/rv32i.test")
        .unwrap()
        .lines()
        .for_each(|line| {
            if let Some((instr_hex, expected)) = line.split_once(' ') {
                let instr = i64::from_str_radix(instr_hex, 16).unwrap();
                if let Ok(iform) = test_decoder.decode_from_i64(instr, 32) {
                    assert_eq!(iform, expected.trim());
                }
            }
        });
}
