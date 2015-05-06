/// A lexer for utf-8 encoded json data
pub struct Lexer<I> 
                where I: Iterator<Item=char> {
    chars: I,
    prev_end: u64,
}

#[derive(Debug, PartialEq, Clone)]
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
    String,
    /// `true`
    BooleanTrue,
    /// `false`
    BooleanFalse,

    /// any json number, like `1.24123` or `123`
    // NOTE: We can't do numbers with our simplified lexer as it would require
    // us to read a char just to see that it's not a number and thus the previous
    // tokens are to be returned. But we cannot peek without drastically complicating
    // our so far quite speedy implementation.
    // Number,
    /// `null`
    Null,
    
    /// The type of the token could not be identified.
    /// Should be removed if this lexer is ever to be feature complete
    Invalid,
}

/// A pair of indices into the character stream returned by our source 
/// iterator.
/// It is an exclusive range.
#[derive(Debug, PartialEq, Clone)]
pub struct Span {
    /// Index of the first the unicode character
    pub first: u64,
    /// Index one past the last unicode character
    pub end: u64,
}

/// A lexical token, identifying its kind and span.
#[derive(Debug, PartialEq, Clone)]
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

        let mut b4 = ['x', 'x', 'x', 'x'];         // buffer (4 characters)
        let mut b5 = ['x', 'x', 'x', 'x', 'x'];    // buffer (5 characters)
        let mut nbi = 0usize;                      // null buffer index
        let mut tbi = 0usize;                      // true buffer index
        let mut fbi = 0usize;                      // false buffer index

        for c in self.chars.by_ref() {
            lcid += 1;

            let mut set_cursor = || {
                first = prev_end + lcid - 1;
            };

            // STRING FAST PATH
            if in_str {
                if ign_next && (c == '"' || c == '\\') {
                    ign_next = false;
                    continue;
                }
                match c {
                    '"' => {
                        t = TokenType::String;
                        break;
                    },
                    '\\' => {
                        ign_next = true;
                        continue;    
                    },
                    _ => {
                        continue;
                    }
                }
            // NULL FAST PATH
            } else if nbi > 0 {
                b4[nbi] = c;
                if nbi == 3 {
                    // we know b4[0] is 'n'
                    if b4[1] == 'u' && b4[2] == 'l' && b4[3] == 'l' {
                        t = TokenType::Null;
                    }
                    break;
                } else {
                    nbi += 1;
                    continue;
                }
            // TRUE FAST PATH
            } else if tbi > 0 {
                b4[tbi] = c;
                if tbi == 3 {
                    // we know b4[0] is 't'
                    if b4[1] == 'r' && b4[2] == 'u' && b4[3] == 'e' {
                        t = TokenType::BooleanTrue;
                    }
                    break;
                } else {
                    tbi += 1;
                    continue;
                }
            // FALSE FAST PATH
            } else if fbi > 0 {
                b5[fbi] = c;
                if fbi == 4 {
                    // we know b5[0] is 'f'
                    if b5[1] == 'a' && b5[2] == 'l' && b5[3] == 's' && b5[4] == 'e' {
                        t = TokenType::BooleanFalse;
                    }
                    break;
                } else {
                    fbi += 1;
                    continue;
                }
            }

            match c {
                '{' => { t = TokenType::CurlyOpen; set_cursor(); break; },
                '}' => { t = TokenType::CurlyClose; set_cursor(); break; },
                '"' => {
                    debug_assert!(!in_str);
                    in_str = true;
                    set_cursor();
                },
                'n' => {
                    debug_assert_eq!(nbi, 0);
                    b4[0] = c;
                    nbi = 1;
                    set_cursor();
                },
                't' => {
                    debug_assert_eq!(tbi, 0);
                    b4[0] = c;
                    tbi = 1;
                    set_cursor();
                },
                'f' => {
                    debug_assert_eq!(fbi, 0);
                    b5[0] = c;
                    fbi = 1;
                    set_cursor();
                },
                '[' => { t = TokenType::BracketOpen; set_cursor(); break; },
                ']' => { t = TokenType::BracketClose; set_cursor(); break; },
                ':' => { t = TokenType::Colon; set_cursor(); break; },
                ',' => { t = TokenType::Comma; set_cursor(); break; },
                '\\' => {
                    // invalid
                    debug_assert_eq!(t, TokenType::Invalid);
                    set_cursor();
                    break
                }
                _ => {
                    // Everything here is considered whitespace, which is skipped
                },
            }// end single character match
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