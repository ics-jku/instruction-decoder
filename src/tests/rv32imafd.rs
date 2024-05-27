use crate::Decoder;
use std::fs::read_to_string;

#[test]
fn test_rv32imfd() {
    let test_decoder = Decoder::new(&vec![
        include_str!("../../examples/RV32I.toml").to_string(),
        include_str!("../../examples/RV32M.toml").to_string(),
        include_str!("../../examples/RV32A.toml").to_string(),
        include_str!("../../examples/RV32F.toml").to_string(),
        include_str!("../../examples/RV32D.toml").to_string(),
    ]);

    read_to_string("src/tests/rv32imafd.test")
        .unwrap()
        .lines()
        .for_each(|line| {
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
                    //test_decoder.decode_from_i64(instr, 32) {
                    assert_eq!(
                        iform,
                        expected.trim(),
                        "Decoding {} does not match expected value",
                        instr_hex
                    );
                }
            }
        });
}
