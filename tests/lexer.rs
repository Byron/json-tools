extern crate json_tools;

use json_tools::{Lexer, Token, Span, TokenType};

#[test]
fn unicode() {
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
    // Add code here
}


#[test]
fn multi_line_strings() {
    // Add code here
}