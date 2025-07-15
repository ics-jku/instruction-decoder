use instruction_decoder::Decoder;

pub fn main() {
    match Decoder::new(&[include_str!("../toml/RV32I.toml").to_string()]) {
        Ok(test_decoder) => {
            let inst = 0xff460593;

            if let Ok(iform) = test_decoder.decode_from_i64(inst, 32) {
                println!("{iform:?}");
            }
        }
        Err(error_stacks) => {
            println!("Errors in ../toml/RV32I.toml:");
            for error in &error_stacks[0] {
                println!("\t{error}");
            }
        }
    }
}
