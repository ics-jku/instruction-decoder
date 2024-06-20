use crate::isa_test;
use crate::Decoder;
use std::fs::read_to_string;

#[test]
fn test_rv64i() {
    isa_test!("../../toml/RV64I.toml", "test_data/rv64/rv64i.test");
}

#[test]
fn test_rv64d() {
    isa_test!("../../toml/RV64D.toml", "test_data/rv64/rv64d.test");
}

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

#[test]
fn test_rv64_zfh() {
    isa_test!("../../toml/RV_Zfh.toml", "test_data/rv64/rv64zfh.test");
}

#[test]
fn test_rv64_zicond() {
    isa_test!(
        "../../toml/RV_Zicond.toml",
        "test_data/rv64/rv64zicond.test"
    );
}

#[test]
fn test_rv64_zifencei() {
    isa_test!(
        "../../toml/RV_Zifencei.toml",
        "test_data/rv64/rv64zifencei.test"
    );
}
