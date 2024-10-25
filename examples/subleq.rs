use instruction_decoder::Decoder;

pub fn main() {
    if let Ok(test_decoder) = Decoder::new(&vec![include_str!("../toml/subleq.toml").to_string()]) {
        let inst = 0x1204;

        if let Ok(iform) = test_decoder.decode_from_i64(inst, 16) {
            println!("{:?}", iform);
        }
    }
}
