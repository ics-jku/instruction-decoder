use crate::isa_test;
use crate::Decoder;
use std::fs::read_to_string;

#[test]
fn test_rv32i() {
    isa_test!("../../examples/RV32I.toml", "test_data/rv32/rv32i.test");
}

#[test]
fn test_rv32m() {
    isa_test!("../../examples/RV32M.toml", "test_data/rv32/rv32m.test");
}

#[test]
fn test_rv32a() {
    isa_test!("../../examples/RV32A.toml", "test_data/rv32/rv32a.test");
}

#[test]
fn test_rv32f() {
    isa_test!("../../examples/RV32F.toml", "test_data/rv32/rv32f.test");
}

#[test]
fn test_rv32d() {
    isa_test!("../../examples/RV32D.toml", "test_data/rv32/rv32d.test");
}

#[test]
fn test_rv32zicsr() {
    isa_test!(
        "../../examples/RV32Zicsr.toml",
        "test_data/rv32/rv32Zicsr.test"
    );
}
