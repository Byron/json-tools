extern crate json_tools;

use json_tools::{Buffer, BufferType, Lexer, Span, Token, TokenType};

#[test]
fn string_value() {
    let src = r#"{ "face": "😂" }"#;
    let mut it = Lexer::new(src.bytes(), BufferType::Span);

    assert_eq!(
        it.next(),
        Some(Token {
            kind: TokenType::CurlyOpen,
            buf: Buffer::Span(Span { first: 0, end: 1 }),
        })
    );
    assert_eq!(
        it.next(),
        Some(Token {
            kind: TokenType::String,
            buf: Buffer::Span(Span { first: 2, end: 8 }),
        })
    );
    assert_eq!(
        it.next(),
        Some(Token {
            kind: TokenType::Colon,
            buf: Buffer::Span(Span { first: 8, end: 9 }),
        })
    );
    assert_eq!(
        it.next(),
        Some(Token {
            kind: TokenType::String,
            buf: Buffer::Span(Span { first: 10, end: 16 }),
        })
    );
    assert_eq!(
        it.next(),
        Some(Token {
            kind: TokenType::CurlyClose,
            buf: Buffer::Span(Span { first: 17, end: 18 }),
        })
    );
}

#[test]
fn string_escaping() {
    let src = r#"{"s":"\"in\""}"#;
    let it = Lexer::new(src.bytes(), BufferType::Span);
    assert_eq!(
        it.skip(3).next(),
        Some(Token {
            kind: TokenType::String,
            buf: Buffer::Span(Span { first: 5, end: 13 }),
        })
    );

    // '\"' makes us ignore the beginning of the string, and we never hit the end
    let src = r#"{"s":\"foo"}"#;
    let mut it = Lexer::new(src.bytes(), BufferType::Span);
    // this is the '\' character - only valid within a string
    assert_eq!(
        it.by_ref().skip(3).next(),
        Some(Token {
            kind: TokenType::Invalid,
            buf: Buffer::Span(Span { first: 5, end: 6 }),
        })
    );
    // now comes the string
    assert_eq!(
        it.next(),
        Some(Token {
            kind: TokenType::String,
            buf: Buffer::Span(Span { first: 6, end: 11 }),
        })
    );

    assert!(it.next().is_some());
    // last one hit the end already˚
    assert!(it.next().is_none());
}

#[test]
fn unclosed_string_value() {
    // '\"' makes us ignore the beginning of the string, and we never hit the end
    let src = r#"{"s":"f}"#;
    let mut it = Lexer::new(src.bytes(), BufferType::Span);

    // unclosed strings are invalid
    assert_eq!(
        it.by_ref().skip(3).next(),
        Some(Token {
            kind: TokenType::Invalid,
            buf: Buffer::Span(Span { first: 5, end: 8 }),
        })
    );
}

#[test]
fn backslash_escapes_backslash_in_string_value() {
    let src = r#"{"s":"f\\"}"#;
    let mut it = Lexer::new(src.bytes(), BufferType::Span);

    assert_eq!(
        it.by_ref().skip(3).next(),
        Some(Token {
            kind: TokenType::String,
            buf: Buffer::Span(Span { first: 5, end: 10 }),
        })
    );

    let src = r#"{"s":"f\"}"#;
    let mut it = Lexer::new(src.bytes(), BufferType::Span);

    assert_eq!(
        it.by_ref().skip(3).next(),
        Some(Token {
            kind: TokenType::Invalid,
            buf: Buffer::Span(Span { first: 5, end: 10 }),
        })
    );
}

#[test]
fn other_backslash_escapes_in_string_value() {
    let src = r#"{"s":"\/\b\f\n\r\t\u1a2B"}"#;
    let mut it = Lexer::new(src.bytes(), BufferType::Span);

    assert_eq!(
        it.by_ref().skip(3).next(),
        Some(Token {
            kind: TokenType::String,
            buf: Buffer::Span(Span { first: 5, end: 25 }),
        })
    );

    let src = r#"{"s":"\a"}"#;
    let mut it = Lexer::new(src.bytes(), BufferType::Span);

    assert_eq!(
        it.by_ref().skip(3).next(),
        Some(Token {
            kind: TokenType::Invalid,
            buf: Buffer::Span(Span { first: 5, end: 8 }),
        })
    );

    let src = r#"{"s":"\u123"}"#;
    let mut it = Lexer::new(src.bytes(), BufferType::Span);

    assert_eq!(
        it.by_ref().skip(3).next(),
        Some(Token {
            kind: TokenType::Invalid,
            buf: Buffer::Span(Span { first: 5, end: 12 }),
        })
    );

    let src = r#"{"s":"\u123x"}"#;
    let mut it = Lexer::new(src.bytes(), BufferType::Span);

    assert_eq!(
        it.by_ref().skip(3).next(),
        Some(Token {
            kind: TokenType::Invalid,
            buf: Buffer::Span(Span { first: 5, end: 12 }),
        })
    );
}

#[test]
fn isolated_value_pairs() {
    for &(src, ref kind, first, end, buf) in &[
        (r#""v":12"#, TokenType::Number, 4, 6, "12"),
        (r#""v":-12"#, TokenType::Number, 4, 7, "-12"),
        (r#""v":"12""#, TokenType::String, 4, 8, r#""12""#),
        (r#""v":true"#, TokenType::BooleanTrue, 4, 8, ""),
        (r#""v":false"#, TokenType::BooleanFalse, 4, 9, ""),
        (r#""v":null"#, TokenType::Null, 4, 8, ""),
    ] {
        let mut it = Lexer::new(src.bytes(), BufferType::Bytes(0));

        assert_eq!(
            it.by_ref().skip(2).next(),
            Some(Token {
                kind: kind.clone(),
                buf: match &kind {
                    TokenType::Number => Buffer::MultiByte(buf.as_bytes().to_vec()),
                    TokenType::String => Buffer::MultiByte(buf.as_bytes().to_vec()),
                    _ => Buffer::Span(Span { first, end }),
                }
            })
        );

        let mut it = Lexer::new(src.bytes(), BufferType::Span);

        assert_eq!(
            it.by_ref().skip(2).next(),
            Some(Token {
                kind: kind.clone(),
                buf: Buffer::Span(Span { first, end }),
            })
        );
    }
}

#[test]
fn special_values_closed_and_unclosed() {
    for &(src, ref kind, first, end) in &[
        (r#"{"v":null}"#, TokenType::Null, 5, 9),
        (r#"{"v":nuxl}"#, TokenType::Invalid, 5, 9),
        (r#"{"v":true}"#, TokenType::BooleanTrue, 5, 9),
        (r#"{"v":trux}"#, TokenType::Invalid, 5, 9),
        (r#"{"v":false}"#, TokenType::BooleanFalse, 5, 10),
        (r#"{"v":falsze}"#, TokenType::Invalid, 5, 10),
        (r#"{"v":123}"#, TokenType::Number, 5, 8),
        (r#"{"v":-123}"#, TokenType::Number, 5, 9),
        (r#"{"v":1.23}"#, TokenType::Number, 5, 9),
        (r#"{"v":-1.23}"#, TokenType::Number, 5, 10),
        (r#"{"v":1.}"#, TokenType::Number, 5, 7),
        (r#"{"v":.}"#, TokenType::Number, 5, 6),
        (r#"{"v":1.23E12}"#, TokenType::Number, 5, 12),
        (r#"{"v":1.23e12}"#, TokenType::Number, 5, 12),
        (r#"{"v":1.23E+12}"#, TokenType::Number, 5, 13),
        (r#"{"v":1.23e+12}"#, TokenType::Number, 5, 13),
        (r#"{"v":1.23E-12}"#, TokenType::Number, 5, 13),
        (r#"{"v":1.23e-12}"#, TokenType::Number, 5, 13),
    ] {
        assert_eq!(
            Lexer::new(src.bytes(), BufferType::Span).skip(3).next(),
            Some(Token {
                kind: kind.clone(),
                buf: Buffer::Span(Span { first: first, end: end }),
            })
        );
    }
}

#[test]
fn whitespace_at_end() {
    assert!(Lexer::new("{} ".bytes(), BufferType::Span).any(|t| t.kind == TokenType::Invalid) == false)
}
