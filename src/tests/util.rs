#[macro_export]
macro_rules! isa_test {
    ($toml:expr, $test:expr) => {
        match Decoder::new(&[include_str!($toml).to_string()]) {
            Ok(test_decoder) => {
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
            }
            Err(error_stacks) => {
                panic!("Got errors when parsing tomls: {:?}", error_stacks)
            }
        }
    };
}

#[macro_export]
macro_rules! error_test {
    ($toml:expr, $error_stack:expr) => {
        match Decoder::new(&[include_str!($toml).to_string()]) {
            Ok(_) => assert!(
                $error_stack.is_empty(),
                "Expected errors {:?} but found none",
                $error_stack,
            ),
            Err(error_stacks) => {
                assert_eq!(
                    $error_stack, error_stacks[0],
                    "Expected errors {:?} but found {:?}",
                    $error_stack, error_stacks[0],
                );
            }
        };
    };
}
