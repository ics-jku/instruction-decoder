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

#[test]
fn test_rv64_zbkb() {
    isa_test!("../../toml/RV64_Zbkb.toml", "test_data/rv64/rv64zbkb.test");
}

#[test]
fn test_rv64_zbkc() {
    isa_test!("../../toml/RV_Zbkc.toml", "test_data/rv64/rvzbkc.test");
}

#[test]
fn test_rv64_zbkx() {
    isa_test!("../../toml/RV_Zbkx.toml", "test_data/rv64/rvzbkx.test");
}

#[test]
fn test_rv64_zkne() {
    isa_test!("../../toml/RV64_Zkne.toml", "test_data/rv64/rv64zkne.test");
}

#[test]
fn test_rv64_zknd() {
    isa_test!("../../toml/RV64_Zknd.toml", "test_data/rv64/rv64zknd.test");
}

#[test]
fn test_rv64_zknh() {
    isa_test!("../../toml/RV_Zknh.toml", "test_data/rv64/rvzknh.test");
}

#[test]
fn test_rv64_zksed() {
    isa_test!("../../toml/RV_Zksed.toml", "test_data/rv64/rvzksed.test");
}

#[test]
fn test_rv64_zksh() {
    isa_test!("../../toml/RV_Zksh.toml", "test_data/rv64/rvzksh.test");
}

#[test]
fn test_rv64_zvbb() {
    isa_test!("../../toml/RV_Zvbb.toml", "test_data/rv64/rvzvbb.test");
}

#[test]
fn test_rv64_zvbc() {
    isa_test!("../../toml/RV_Zvbc.toml", "test_data/rv64/rvzvbc.test");
}

#[test]
fn test_rv64_zvkg() {
    isa_test!("../../toml/RV_Zvkg.toml", "test_data/rv64/rvzvkg.test");
}

#[test]
fn test_rv64_zvkned() {
    isa_test!("../../toml/RV_Zvkned.toml", "test_data/rv64/rvzvkned.test");
}

#[test]
fn test_rv64_zvknha() {
    isa_test!("../../toml/RV_Zvknha.toml", "test_data/rv64/rvzvknha.test");
}

#[test]
fn test_rv64_zvknhb() {
    isa_test!("../../toml/RV_Zvknhb.toml", "test_data/rv64/rvzvknhb.test");
}

#[test]
fn test_rv64_zvksed() {
    isa_test!("../../toml/RV_Zvksed.toml", "test_data/rv64/rvzvksed.test");
}

#[test]
fn test_rv64_zvksh() {
    isa_test!("../../toml/RV_Zvksh.toml", "test_data/rv64/rvzvksh.test");
}

#[test]
fn test_rv64_zawrs() {
    isa_test!("../../toml/RV_Zawrs.toml", "test_data/rv64/rvzawrs.test");
}

#[test]
fn test_rv64_c() {
    isa_test!("../../toml/RV64C.toml", "test_data/rv64/rv64c.test");
}

#[test]
fn test_rv64_zcd() {
    isa_test!("../../toml/RV64Zcd.toml", "test_data/rv64/rv64zcd.test");
}
