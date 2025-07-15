use instruction_decoder::Decoder;

pub fn main() {
    match Decoder::new(&[include_str!("../toml/subleq.toml").to_string()]) {
        Ok(test_decoder) => {
            let inst = 0x1204;

            if let Ok(iform) = test_decoder.decode_from_i64(inst, 32) {
                println!("{iform:?}");
            }
        }
        Err(error_stacks) => {
            println!("Errors in ../toml/subleq.toml:");
            for error in &error_stacks[0] {
                println!("\t{error}");
            }
        }
    }
}
