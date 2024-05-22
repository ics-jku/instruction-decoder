example useage:

```
use instruction_decoder::Decoder;

pub fn main() {
    let mut test_decoder = Decoder::new(&vec![include_str!("RV32I.toml").to_string()]);
    let inst = 0xff460593;

    if let Ok(iform) = test_decoder.decode_from_i64(inst, 32) {
        println!("{:?}", iform);
    }
}
```