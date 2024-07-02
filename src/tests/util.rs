#[macro_export]
macro_rules! isa_test {
    ($toml:expr, $test:expr) => {
        let test_decoder = Decoder::new(&vec![include_str!($toml).to_string()]);
        read_to_string($test).unwrap().lines().for_each(|line| {
            if let Some((instr_hex, expected)) = line.split_once(' ') {
                let instr = i64::from_str_radix(instr_hex, 16).unwrap();
                let iform = test_decoder.decode_from_i64(
                    instr,
                    test_decoder.instruction_sets.get(0).unwrap().bit_width,
                );
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
