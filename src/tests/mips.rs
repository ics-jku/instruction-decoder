use crate::isa_test;
use crate::Decoder;
use std::fs::read_to_string;

#[test]
fn test_mips() {
    &[&[isa_test!("../../toml/mips.toml", "test_data/mips/mips.test")]];
}
