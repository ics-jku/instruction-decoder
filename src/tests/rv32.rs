use crate::Decoder;
use std::fs::read_to_string;

macro_rules! isa_test {
    ($toml:expr, $test:expr) => {
        let test_decoder = Decoder::new(&vec![include_str!($toml).to_string()]);
        read_to_string($test).unwrap().lines().for_each(|line| {
            if let Some((instr_hex, expected)) = line.split_once(' ') {
                let instr = i64::from_str_radix(instr_hex, 16).unwrap();
                let iform = test_decoder.decode_from_i64(instr, 32);
                assert!(
                    iform.is_ok(),
                    "Can not decode {} into expected {}",
                    instr_hex,
                    expected.trim()
                );
                if let Ok(iform) = iform {
                    assert_eq!(
                        iform,
                        expected.trim(),
                        "Decoding {} does not match expected value",
                        instr_hex
                    );
                }
            }
        });
    };
}

#[test]
fn test_rv32i() {
    isa_test!("../../examples/RV32I.toml", "src/tests/data/rv32i.test");
}

#[test]
fn test_rv32m() {
    isa_test!("../../examples/RV32M.toml", "src/tests/data/rv32m.test");
}

#[test]
fn test_rv32a() {
    isa_test!("../../examples/RV32A.toml", "src/tests/data/rv32a.test");
}

#[test]
fn test_rv32f() {
    isa_test!("../../examples/RV32F.toml", "src/tests/data/rv32f.test");
}

#[test]
fn test_rv32d() {
    isa_test!("../../examples/RV32D.toml", "src/tests/data/rv32d.test");
}
