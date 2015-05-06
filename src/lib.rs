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

        let mut nb = ['x', 'x', 'x', 'x']; // null buffer
        let mut nbi = 0usize;                 // null buffer index

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
                        t = TokenType::StringValue;
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
                nb[nbi] = c;
                if nbi == 3 {
                    // we know nb[0] is 'n'
                    if nb[1] == 'u' && nb[2] == 'l' && nb[3] == 'l' {
                        t = TokenType::NullValue;
                    }
                    break;
                } else {
                    nbi += 1;
                    continue;
                }
            }

            match c {
                '\\' => {
                    // invalid
                    debug_assert_eq!(t, TokenType::Invalid);
                    set_cursor();
                    break
                }
                '{' => { t = TokenType::CurlyOpen; set_cursor(); break; },
                '}' => { t = TokenType::CurlyClose; set_cursor(); break; },
                '[' => { t = TokenType::BracketOpen; set_cursor(); break; },
                ']' => { t = TokenType::BracketClose; set_cursor(); break; },
                ':' => { t = TokenType::Colon; set_cursor(); break; },
                ',' => { t = TokenType::Comma; set_cursor(); break; },
                'n' => {
                    debug_assert_eq!(nbi, 0);
                    nb[0] = c;
                    nbi = 1;
                    set_cursor();
                }
                '"' => {
                    debug_assert!(!in_str);
                    in_str = true;
                    set_cursor();
                },
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