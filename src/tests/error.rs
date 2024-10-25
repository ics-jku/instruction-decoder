use crate::error_test;
use crate::Decoder;

#[test]
fn test_no_error() {
    error_test!("../../toml/mips.toml", Vec::<String>::new());
}

#[test]
fn test_some_error() {
    error_test!(
        "../../toml-with-errors/mips.toml",
        vec!["key 'types.B' not found in toml".to_string()]
    );
}
