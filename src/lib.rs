/// A lexer for utf-8 encoded json data
pub struct Lexer<I> 
                where I: Iterator<Item=char> {
    chars: I,
    prev_end: u64,
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    /// `{`
    CurlyOpen,
    /// `}`
    CurlyClose,

    /// `[`
    BracketOpen,
    /// `]`
    BracketClose,

    /// `:`
    Colon,
    /// `,`
    Comma,


    /// A json string , like `"foo"`
    StringValue,
    /// `true`
    BooleanTrue,
    /// `false`
    BooleanFalse,
    /// any json number, like `1.24123` or `123`
    Number,
    /// `null`
    NullValue,
    
    /// The type of the token could not be identified.
    /// Should be removed if this lexer is ever to be feature complete
    Invalid,
}

/// A pair of indices into the character stream returned by our source 
/// iterator.
/// It is an exclusive range.
#[derive(Debug, PartialEq)]
pub struct Span {
    /// Index of the first the unicode character
    pub first: u64,
    /// Index one past the last unicode character
    pub end: u64,
}

/// A lexical token, identifying its kind and span.
#[derive(Debug, PartialEq)]
pub struct Token {
    /// The exact type of the token
    pub kind: TokenType,
    /// The span allows to reference back into the source character stream 
    /// to obtain the string making up the token.
    pub span: Span,
}

impl<I> Lexer<I> where I: Iterator<Item=char> {
    /// Returns a new Lexer from a given character iterator.
    pub fn new(chars: I) -> Lexer<I> {
        Lexer {
            chars: chars,
            prev_end: 0,
        }
    }
}

impl<I> Iterator for Lexer<I> 
                    where I: Iterator<Item=char> {
    type Item = Token;

    /// Lex the underlying character stream to generate tokens
    fn next(&mut self) -> Option<Token> {
        let mut t: TokenType = TokenType::Invalid;
        let mut lcid: u64 = 0;

        let mut in_str = false;     // true if we are lexing a string
        let mut ign_next = false;   // ignore next special character meaning

        let mut first = 0;
        let prev_end = self.prev_end;

        for c in self.chars.by_ref() {
            lcid += 1;

            let mut set_cursor = || {
                first = prev_end + lcid - 1;
            };

            match c {
                '\\' => {
                    if in_str {
                        ign_next = true;
                        continue; // this is allowed only here !
                    } else {
                        // invalid
                        assert_eq!(t, TokenType::Invalid);
                        set_cursor();
                        break
                    }
                }
                '{'
                |'}'
                |'['
                |']'
                |':'
                |',' => {
                    if !in_str {
                        set_cursor();
                        t = match c {
                            '{' => TokenType::CurlyOpen,
                            '}' => TokenType::CurlyClose,
                            '[' => TokenType::BracketOpen,
                            ']' => TokenType::BracketClose,
                            ':' => TokenType::Colon,
                            ',' => TokenType::Comma,
                            _ => unreachable!(),
                        };
                        break;
                    }
                },
                '"' => {
                    if !ign_next {
                        if in_str {
                            break;
                        } else {
                            in_str = true;
                            t = TokenType::StringValue;
                            set_cursor();
                        }
                    }
                },
                _ => {
                    // Everything here is considered whitespace, which is skipped
                },
            }// end single character match

            // NOTE: MUST NOT CONTINUE !
            ign_next = false;
        }// end for each character

        if lcid == 0 {
            None
        } else {
            self.prev_end = self.prev_end + lcid;
            Some(Token {
                kind: t,
                span: Span {
                    first: first,
                    end: self.prev_end
                }
            })
        }
    }
}