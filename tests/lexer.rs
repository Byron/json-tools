extern crate json_tools;

use json_tools::{Lexer, Token, Span, TokenType};

#[test]
fn unicode() {
    let src = r#"{ "face": "😂" }"#;
    let mut it = Lexer::new(src.chars());

    assert_eq!(it.next(), Some(Token { kind: TokenType::BracketLeft, 
                                       span: Span { first: 0,
                                                    end:  1 } }));
}


#[test]
fn string_escaping() {
    // Add code here
}


#[test]
fn multi_line_strings() {
    // Add code here
}