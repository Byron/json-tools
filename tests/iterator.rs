extern crate json_tools;

use json_tools::{Lexer, FilterNull};

#[test]
fn filter_null_values() {
    let src = r#"{"s":null, "v":true, "s":null }"#;
    assert_eq!(Lexer::new(src.chars()).count(), 13);
    assert_eq!(FilterNull::new(Lexer::new(src.chars())).count(), 5);
}