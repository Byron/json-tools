extern crate json_tools;

use json_tools::{Lexer, Token, Span, TokenType};

#[test]
fn string_value() {
    let src = r#"{ "face": "😂" }"#;
    let mut it = Lexer::new(src.chars());

    assert_eq!(it.next(), Some(Token { kind: TokenType::CurlyOpen, 
                                       span: Span { first: 0,
                                                    end:  1 } }));
    assert_eq!(it.next(), Some(Token { kind: TokenType::StringValue, 
                                       span: Span { first: 2,
                                                    end:  8 } }));
    assert_eq!(it.next(), Some(Token { kind: TokenType::Colon, 
                                       span: Span { first: 8,
                                                    end:  9 } }));
    assert_eq!(it.next(), Some(Token { kind: TokenType::StringValue, 
                                       span: Span { first: 10,
                                                    end:  13 } }));
    assert_eq!(it.next(), Some(Token { kind: TokenType::CurlyClose, 
                                       span: Span { first: 14,
                                                    end:  15 } }));
}


#[test]
fn string_escaping() {
    let src = r#"{"s":"\"in\""}"#;
    let it = Lexer::new(src.chars());
    assert_eq!(it.skip(3).next(), Some(Token { kind: TokenType::StringValue, 
                                               span: Span { first: 5,
                                                            end:  13 } }));

    // '\"' makes us ignore the beginning of the string, and we never hit the end
    let src = r#"{"s":\"foo"}"#;
    let mut it = Lexer::new(src.chars());
    // this is the '\' character - only valid within a string
    assert_eq!(it.by_ref().skip(3).next(), Some(Token { kind: TokenType::Invalid, 
                                               span: Span { first: 5,
                                                            end:  6 } }));
    // now comes the string
    assert_eq!(it.next(), Some(Token { kind: TokenType::StringValue, 
                                               span: Span { first: 6,
                                               end:  11 } }));
    
    assert!(it.next().is_some());
    // last one hit the end already˚
    assert!(it.next().is_none());
}

#[test]
fn unclosed_string_value() {
    // '\"' makes us ignore the beginning of the string, and we never hit the end
    let src = r#"{"s":"f}"#;
    let mut it = Lexer::new(src.chars());

    // unclosed strings are invalid
    assert_eq!(it.by_ref().skip(3).next(), Some(Token { kind: TokenType::Invalid, 
                                                        span: Span { first: 5,
                                                                     end:  8 } }));
}


#[test]
fn null_value() {
    let src = r#"{"n":null}"#;
    let mut it = Lexer::new(src.chars());

    assert_eq!(it.by_ref().skip(3).next(), Some(Token { kind: TokenType::NullValue, 
                                                        span: Span { first: 5,
                                                                     end:  9 } }));
}

#[test]
fn unclosed_null_value() {
    let src = r#"{"n":nuxl}"#;
    let mut it = Lexer::new(src.chars());

    // invalid null characters cause it to be invalid
    assert_eq!(it.by_ref().skip(3).next(), Some(Token { kind: TokenType::Invalid, 
                                                        span: Span { first: 5,
                                                                     end:  9 } }));
}

#[test]
fn multi_line_strings() {
    // Add code here
}