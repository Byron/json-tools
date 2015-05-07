extern crate json_tools;

use json_tools::{Lexer, FilterNull};

#[test]
fn filter_null_values() {
    for &(src, count, fcount) in &[(r#"{"s":null, "s":true, "s":null }"#, 13, 5),
                                   (r#"{"s":null, "s":null, "s":null }"#, 13, 2),
                                   (r#"{"s":true, "s":null, "s":null }"#, 13, 5),
                                   (r#"{"s":true, "s":null "s":null }"#, 12, 5), // invalid is fine
                                   (r#"{"s":true,,,, "s":null, "s":null }"#, 16, 8),
                                   (r#"{"s":null, "s":null, "s":true }"#, 13, 5),
                                   (r#"{"s":true, "s":null, "s":true }"#, 13, 9),
                                   (r#"{"s":true, "s":null "s":true }"#, 12, 8),] {
        assert_eq!(Lexer::new(src.chars()).count(), count);
        assert_eq!(FilterNull::new(Lexer::new(src.chars())).count(), fcount);    
    }
}