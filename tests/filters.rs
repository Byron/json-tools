extern crate json_tools;

use json_tools::{Lexer, FilterNull};

#[test]
fn filter_null_values() {
    for &(src, count, fcount) in &[(r#"{"s":null, "s":true, "s":null }"#, 13, 6), //6 should be 5!
                                   (r#"{"s":null, "s":null, "s":null }"#, 13, 2),
                                   (r#"{"s":true, "s":null, "s":null }"#, 13, 6),
                                   (r#"{"s":null, "s":null, "s":true }"#, 13, 5)] {
        assert_eq!(Lexer::new(src.chars()).count(), count);
        assert_eq!(FilterNull::new(Lexer::new(src.chars())).count(), fcount);    
    }
}