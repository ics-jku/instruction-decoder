use crate::isa_test;
use crate::Decoder;
use std::fs::read_to_string;

#[test]
fn test_rv32i() {
    isa_test!("../../toml/RV32I.toml", "test_data/rv32/rv32i.test");
}

#[test]
fn test_rv32m() {
    isa_test!("../../toml/RV32M.toml", "test_data/rv32/rv32m.test");
}

#[test]
fn test_rv32a() {
    isa_test!("../../toml/RV32A.toml", "test_data/rv32/rv32a.test");
}

#[test]
fn test_rv32f() {
    isa_test!("../../toml/RV32F.toml", "test_data/rv32/rv32f.test");
}

#[test]
fn test_rv32zicsr() {
    isa_test!(
        "../../toml/RV32_Zicsr.toml",
        "test_data/rv32/rv32zicsr.test"
    );
}

#[test]
fn test_rv32_zbb() {
    isa_test!("../../toml/RV32_Zbb.toml", "test_data/rv32/rv32zbb.test");
}

#[test]
fn test_rv32_zbs() {
    isa_test!("../../toml/RV32_Zbs.toml", "test_data/rv32/rv32zbs.test");
}

#[test]
fn test_rv32_zbkb() {
    isa_test!("../../toml/RV32_Zbkb.toml", "test_data/rv32/rv32zbkb.test");
}

#[test]
fn test_rv32_zkne() {
    isa_test!("../../toml/RV32_Zkne.toml", "test_data/rv32/rv32zkne.test");
}

#[test]
fn test_rv32_c() {
    isa_test!("../../toml/RV32C.toml", "test_data/rv32/rv32c.test");
}

#[test]
fn test_rv32_zfa() {
    isa_test!("../../toml/RV32_Zfa.toml", "test_data/rv32/rv32zfa.test");
}
