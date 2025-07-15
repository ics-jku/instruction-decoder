use crate::isa_test;
use crate::Decoder;
use std::fs::read_to_string;

#[test]
fn test_la64() {
    &[&[isa_test!("../../toml/la64.toml", "test_data/la64/la64.test")]];
}
