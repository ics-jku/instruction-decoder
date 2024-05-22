use instruction_decoder::Decoder;

pub fn main() {
    let mut test_decoder = Decoder::new(&vec![include_str!("subleq.toml").to_string()]);
    let inst = 0x1204;

    if let Ok(iform) = test_decoder.decode_from_i64(inst, 16) {
        println!("{:?}", iform);
    }
}
