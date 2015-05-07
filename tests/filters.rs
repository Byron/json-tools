extern crate json_tools;

use std::io::{Read, self, Write};
use json_tools::{Lexer, FilterTypedKeyValuePairs, BufferType, TokenType, TokenReader};

#[test]
fn filter_null_values() {
    for &(src, want, count, fcount) in 
                                  &[(r#"{  "s":null, "s":true, "s":null }"#,
                                    r#"{"s":true}"#, 13, 5),
                                   (r#"{"s " : null, "s":null, "s":null }"#,
                                    r#"{}"#, 13, 2),
                                   (r#"{"s":true, "s":null, "s":null }"#,
                                    r#"{"s":true}"#, 13, 5),
                                   (r#"{"s":true, "s":null "s":null }"#, 
                                    r#"{"s":true}"#, 12, 5), // invalid is fine
                                   (r#"{"s":true,,,, "s":null, "s":null }"#,
                                    r#"{"s":true,,,}"#, 16, 8),
                                   (r#"{"s":null, "s":null, "s":true }"#,
                                    r#"{"s":true}"#, 13, 5),
                                   (r#"{"s":true, "s":null, "s":true }"#,
                                    r#"{"s":true,"s":true}"#, 13, 9),
                                   (r#"{"s":true, "s":null "s":true }"#,
                                    r#"{"s":true"s":true}"#, 12, 8),] {
        assert_eq!(Lexer::new(src.bytes(), BufferType::Span).count(), count);
        let new_filter = |bt| {
            FilterTypedKeyValuePairs::new(Lexer::new(src.bytes(), bt), TokenType::Null)
        };
        assert_eq!(new_filter(BufferType::Span).count(), fcount);

        for bt in &[BufferType::Bytes(128), BufferType::Span] {
            let mut buf: Vec<u8> = Vec::new();
            let mut byte = [0u8];
            let mut r = TokenReader::new(new_filter(bt.clone()), Some(src));

            while let Ok(l) = r.read(&mut byte) {
                if l == 0 {
                    break
                }
                buf.push(byte[0]);
                // writeln!(io::stderr(), "{:?}", buf).ok();
            }
            assert_eq!(&String::from_utf8(buf).unwrap(), want);
        }
    }
}