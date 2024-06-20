use crate::isa_test;
use crate::Decoder;
use std::fs::read_to_string;

#[test]
fn test_rv64_zba() {
    isa_test!("../../toml/RV_Zba.toml", "test_data/rv64/rvzba.test");
}

#[test]
fn test_rv64_zbb() {
    isa_test!("../../toml/RV64_Zbb.toml", "test_data/rv64/rv64zbb.test");
}

#[test]
fn test_rv64_zbc() {
    isa_test!("../../toml/RV_Zbc.toml", "test_data/rv64/rvzbc.test");
}

#[test]
fn test_rv64_zbs() {
    isa_test!("../../toml/RV64_Zbs.toml", "test_data/rv64/rv64zbs.test");
}
