#![allow(non_upper_case_globals)]

pub mod rv32 {
    pub const RV32I: &str = include_str!("../toml/RV32I.toml");
    pub const RV32M: &str = include_str!("../toml/RV32M.toml");
    pub const RV32A: &str = include_str!("../toml/RV32A.toml");
    pub const RV32F: &str = include_str!("../toml/RV32F.toml");
    pub const RV32_Zbb: &str = include_str!("../toml/RV32_Zbb.toml");
    pub const RV32_Zbkb: &str = include_str!("../toml/RV32_Zbkb.toml");
    pub const RV32_Zbs: &str = include_str!("../toml/RV32_Zbs.toml");
    pub const RV32_Zknd: &str = include_str!("../toml/RV32_Zknd.toml");
    pub const RV32_Zkne: &str = include_str!("../toml/RV32_Zkne.toml");
    pub const RV32_Zfa: &str = include_str!("../toml/RV32_Zfa.toml");
    pub const RV32_Zicsr: &str = include_str!("../toml/RV32_Zicsr.toml");
    pub const RV32C_LOWER: &str = include_str!("../toml/RV32C-lower.toml");
    pub const RV32_Zcb_lower: &str = include_str!("../toml/RV32_Zcb-lower.toml");
    pub const RV32_Zcf_lower: &str = include_str!("../toml/RV32_Zcf-lower.toml");
    pub const RV32_Zacas: &str = include_str!("../toml/RV32_Zacas.toml");
    pub const RV_Zcd_lower: &str = include_str!("../toml/RV_Zcd-lower.toml");
    pub const RV_Zba: &str = include_str!("../toml/RV_Zba.toml");
    pub const RV_Zbc: &str = include_str!("../toml/RV_Zbc.toml");
    pub const RV_Zbkc: &str = include_str!("../toml/RV_Zbkc.toml");
    pub const RV_Zbkx: &str = include_str!("../toml/RV_Zbkx.toml");
    pub const RV_Zfh: &str = include_str!("../toml/RV_Zfh.toml");
    pub const RV_Zknh: &str = include_str!("../toml/RV_Zknh.toml");
    pub const RV_Zksed: &str = include_str!("../toml/RV_Zksed.toml");
    pub const RV_Zksh: &str = include_str!("../toml/RV_Zksh.toml");
    pub const RV_Zawrs: &str = include_str!("../toml/RV_Zawrs.toml");
    pub const RV_Zicond: &str = include_str!("../toml/RV_Zicond.toml");
    pub const RV_Zifencei: &str = include_str!("../toml/RV_Zifencei.toml");
    pub const RV_Zicbo: &str = include_str!("../toml/RV_Zicbo.toml");
    pub const RV_Zimop: &str = include_str!("../toml/RV_Zimop.toml");
    pub const RV_Zihintntl: &str = include_str!("../toml/RV_Zihintntl.toml");
    pub const RVV: &str = include_str!("../toml/RVV.toml");
    pub const RV32: [&str; 32] = [
        RV32I,
        RV32I,
        RV32M,
        RV32A,
        RV32F,
        RV32_Zbb,
        RV32_Zbkb,
        RV32_Zbs,
        RV32_Zknd,
        RV32_Zkne,
        RV32_Zfa,
        RV32_Zicsr,
        RV32C_LOWER,
        RV32_Zcb_lower,
        RV32_Zcf_lower,
        RV32_Zacas,
        RV_Zcd_lower,
        RV_Zba,
        RV_Zbc,
        RV_Zbkc,
        RV_Zbkx,
        RV_Zfh,
        RV_Zknh,
        RV_Zksed,
        RV_Zksh,
        RV_Zawrs,
        RV_Zicond,
        RV_Zifencei,
        RV_Zicbo,
        RV_Zimop,
        RV_Zihintntl,
        RVV,
    ];
}

pub mod rv64 {
    pub const RV64I: &str = include_str!("../toml/RV64I.toml");
    pub const RV64M: &str = include_str!("../toml/RV64M.toml");
    pub const RV64A: &str = include_str!("../toml/RV64A.toml");
    pub const RV64D: &str = include_str!("../toml/RV64D.toml");
    pub const RV64_Zbb: &str = include_str!("../toml/RV64_Zbb.toml");
    pub const RV64_Zbkb: &str = include_str!("../toml/RV64_Zbkb.toml");
    pub const RV64_Zbs: &str = include_str!("../toml/RV64_Zbs.toml");
    pub const RV64_Zknd: &str = include_str!("../toml/RV64_Zknd.toml");
    pub const RV64_Zkne: &str = include_str!("../toml/RV64_Zkne.toml");
    pub const RV64_Zacas: &str = include_str!("../toml/RV64_Zacas.toml");
    pub const RV64_Zfa: &str = include_str!("../toml/RV64_Zfa.toml");
    pub const RV64C_lower: &str = include_str!("../toml/RV64C-lower.toml");
    pub const RV64_Zcb_lower: &str = include_str!("../toml/RV64_Zcb-lower.toml");
    pub const RV64_Zcd_lower: &str = include_str!("../toml/RV64_Zcd-lower.toml");
    pub const RVV: &str = include_str!("../toml/RVV.toml");
    pub const RV_Zvbb: &str = include_str!("../toml/RV_Zvbb.toml");
    pub const RV_Zvbc: &str = include_str!("../toml/RV_Zvbc.toml");
    pub const RV_Zvkg: &str = include_str!("../toml/RV_Zvkg.toml");
    pub const RV_Zvkned: &str = include_str!("../toml/RV_Zvkned.toml");
    pub const RV_Zvknha: &str = include_str!("../toml/RV_Zvknha.toml");
    pub const RV_Zvknhb: &str = include_str!("../toml/RV_Zvknhb.toml");
    pub const RV_Zvksed: &str = include_str!("../toml/RV_Zvksed.toml");
    pub const RV_Zvksh: &str = include_str!("../toml/RV_Zvksh.toml");
    pub const RV_Zcd_lower: &str = include_str!("../toml/RV_Zcd-lower.toml");
    pub const RV_Zba: &str = include_str!("../toml/RV_Zba.toml");
    pub const RV_Zbc: &str = include_str!("../toml/RV_Zbc.toml");
    pub const RV_Zbkc: &str = include_str!("../toml/RV_Zbkc.toml");
    pub const RV_Zbkx: &str = include_str!("../toml/RV_Zbkx.toml");
    pub const RV_Zfh: &str = include_str!("../toml/RV_Zfh.toml");
    pub const RV_Zknh: &str = include_str!("../toml/RV_Zknh.toml");
    pub const RV_Zksed: &str = include_str!("../toml/RV_Zksed.toml");
    pub const RV_Zksh: &str = include_str!("../toml/RV_Zksh.toml");
    pub const RV_Zawrs: &str = include_str!("../toml/RV_Zawrs.toml");
    pub const RV_Zicond: &str = include_str!("../toml/RV_Zicond.toml");
    pub const RV_Zifencei: &str = include_str!("../toml/RV_Zifencei.toml");
    pub const RV_Zicbo: &str = include_str!("../toml/RV_Zicbo.toml");
    pub const RV_Zimop: &str = include_str!("../toml/RV_Zimop.toml");
    pub const RV_Zihintntl: &str = include_str!("../toml/RV_Zihintntl.toml");
    pub const RV64: [&str; 38] = [
        RV64I,
        RV64M,
        RV64A,
        RV64D,
        RV64_Zbb,
        RV64_Zbkb,
        RV64_Zbs,
        RV64_Zknd,
        RV64_Zkne,
        RV64_Zacas,
        RV64_Zfa,
        RV64C_lower,
        RV64_Zcb_lower,
        RV64_Zcd_lower,
        RVV,
        RV_Zvbb,
        RV_Zvbc,
        RV_Zvkg,
        RV_Zvkned,
        RV_Zvknha,
        RV_Zvknhb,
        RV_Zvksed,
        RV_Zvksh,
        RV_Zcd_lower,
        RV_Zba,
        RV_Zbc,
        RV_Zbkc,
        RV_Zbkx,
        RV_Zfh,
        RV_Zknh,
        RV_Zksed,
        RV_Zksh,
        RV_Zawrs,
        RV_Zicond,
        RV_Zifencei,
        RV_Zicbo,
        RV_Zimop,
        RV_Zihintntl,
    ];
}

pub const MIPS: &str = include_str!("../toml/mips.toml");

pub const LA64: &str = include_str!("../toml/la64.toml");
