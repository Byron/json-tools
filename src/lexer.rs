/// A lexer for utf-8 encoded json data
pub struct Lexer<I: Iterator<Item=u8>> {
    chars: I,
    next_byte: Option<u8>,
    cursor: u64,
    buffer_type: BufferType,
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
    /// A Number, like `1.1234` or `123` or `-0.0` or `-1` or `.0` or `.`
    Number,

    /// any json number, like `1.24123` or `123`
    // NOTE: We can't do numbers with our simplified lexer as it would require
    // us to read a byte just to see that it's not a number and thus the previous
    // tokens are to be returned. But we cannot peek without drastically complicating
    // our so far quite speedy implementation.
    // Number,
    /// `null`
    Null,
    
    /// The type of the token could not be identified.
    /// Should be removed if this lexer is ever to be feature complete
    Invalid,
}

/// A pair of indices into the byte stream returned by our source 
/// iterator.
/// It is an exclusive range.
#[derive(Debug, PartialEq, Clone)]
pub struct Span {
    /// Index of the first the byte
    pub first: u64,
    /// Index one past the last byte
    pub end: u64,
}

/// A lexical token, identifying its kind and span.
#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    /// The exact type of the token
    pub kind: TokenType,

    /// A buffer representing the bytes of this Token. 
    pub buf: Buffer
}

/// Representation of a buffer containing items making up a `Token`.
///
/// It's either always `Span`, or one of the `*Byte` variants.
#[derive(Debug, PartialEq, Clone)]
pub enum Buffer {
    /// Multiple bytes making up a token. Only set for `TokenType::String` and 
    /// `TokenType::Number`.
    MultiByte(Vec<u8>),
    /// The span allows to reference back into the source byte stream 
    /// to obtain the string making up the token.
    /// Please note that for control characters, booleans and null (i.e
    /// anything that is not `Buffer::MultiByte` you should use 
    /// `<TokenType as AsRef<str>>::as_ref()`)
    Span(Span),
}

/// The type of `Buffer` you want in each `Token`
pub enum BufferType {
    Bytes,
    Span,
}

impl<I> Lexer<I> where I: Iterator<Item=u8> {
    /// Returns a new Lexer from a given byte iterator.
    pub fn new(chars: I, buffer_type: BufferType) -> Lexer<I> {
        Lexer {
            chars: chars,
            next_byte: None,
            cursor: 0,
            buffer_type: buffer_type,
        }
    }

    pub fn into_inner(self) -> I {
        self.chars
    }

    fn put_back(&mut self, c: u8) {
        debug_assert!(self.next_byte.is_none());
        self.next_byte = Some(c);
        self.cursor -= 1;
    }

    fn next_byte(&mut self) -> Option<u8> {
        match self.next_byte.take() {
            Some(c) => {
                self.cursor += 1;
                Some(c)
            },
            None => {
                let res = self.chars.next();
                match res {
                    None => None,
                    Some(_) => {
                        self.cursor += 1;
                        res
                    }
                }
            }
        }
    }
}

// Identifies the state of the lexer
enum Mode {
    // String parse mode: bool = ignore_next
    String(bool),
    // `null` parse mode: buf, buf-index
    Null([u8; 4], usize),
    // `true` parse mode
    True([u8; 4], usize),
    // `false` parse mode
    False([u8; 5], usize),
    // `Number` parse mode
    Number,
    SlowPath,
}

impl<I> Iterator for Lexer<I> 
                    where I: Iterator<Item=u8> {
    type Item = Token;

    /// Lex the underlying bytte stream to generate tokens
    fn next(&mut self) -> Option<Token> {
        let mut t: TokenType = TokenType::Invalid;

        let mut first = 0;
        let mut state = Mode::SlowPath;
        let last_cursor = self.cursor;
        let mut buf: Vec<u8> = Vec::with_capacity(128);

        while let Some(c) = self.next_byte() {
            if let BufferType::Bytes = self.buffer_type {
                buf.push(c);
            }
            let mut set_cursor = |cursor| {
                first = cursor - 1;
            };

            match state {
                Mode::String(ref mut ign_next) => {
                    if *ign_next && (c == b'"' || c == b'\\') {
                        *ign_next = false;
                        continue;
                    }
                    match c {
                        b'"' => {
                            t = TokenType::String;
                            break;
                        },
                        b'\\' => {
                            *ign_next = true;
                            continue;    
                        },
                        _ => {
                            continue;
                        }
                    }
                },
                Mode::Null(ref mut b, ref mut i) => {
                    b[*i] = c;
                    if *i == 3 {
                        // we know b[0] is b'n'
                        if b[1] == b'u' && b[2] == b'l' && b[3] == b'l' {
                            t = TokenType::Null;
                        }
                        break;
                    } else {
                        *i += 1;
                        continue;
                    }
                },
                Mode::Number => {
                    match c {
                         b'0' ... b'9'
                        |b'-'
                        |b'.' => {
                            continue;
                        },
                        _ => {
                            t = TokenType::Number;
                            self.put_back(c);
                            break;
                        }
                    }
                }
                Mode::True(ref mut b, ref mut i) => {
                    b[*i] = c;
                    if *i == 3 {
                        // we know b[0] is b't'
                        if b[1] == b'r' && b[2] == b'u' && b[3] == b'e' {
                            t = TokenType::BooleanTrue;
                        }
                        break;
                    } else {
                        *i += 1;
                        continue;
                    }
                },
                Mode::False(ref mut b, ref mut i) => {
                    b[*i] = c;
                    if *i == 4 {
                        // we know b[0] is b'f'
                        if b[1] == b'a' && b[2] == b'l' && b[3] == b's' && b[4] == b'e' {
                            t = TokenType::BooleanFalse;
                        }
                        break;
                    } else {
                        *i += 1;
                        continue;
                    }
                },
                Mode::SlowPath => {
                    match c {
                        b'{' => { t = TokenType::CurlyOpen; set_cursor(self.cursor); break; },
                        b'}' => { t = TokenType::CurlyClose; set_cursor(self.cursor); break; },
                        b'"' => {
                            state = Mode::String(false);
                            set_cursor(self.cursor);
                        },
                        b'n' => {
                            state = Mode::Null([c, b'x', b'x', b'x'], 1);
                            set_cursor(self.cursor);
                        },
                         b'0' ... b'9'
                        |b'-'
                        |b'.'=> {
                            state = Mode::Number;
                            set_cursor(self.cursor);
                        },
                        b't' => {
                            state = Mode::True([c, b'x', b'x', b'x'], 1);
                            set_cursor(self.cursor);
                        },
                        b'f' => {
                            state = Mode::False([c, b'x', b'x', b'x', b'x'], 1);
                            set_cursor(self.cursor);
                        },
                        b'[' => { t = TokenType::BracketOpen; set_cursor(self.cursor); break; },
                        b']' => { t = TokenType::BracketClose; set_cursor(self.cursor); break; },
                        b':' => { t = TokenType::Colon; set_cursor(self.cursor); break; },
                        b',' => { t = TokenType::Comma; set_cursor(self.cursor); break; },
                        b'\\' => {
                            // invalid
                            debug_assert_eq!(t, TokenType::Invalid);
                            set_cursor(self.cursor);
                            break
                        }
                        _ => {
                            // Everything here is considered whitespace, which is skipped
                            if let BufferType::Bytes = self.buffer_type {
                                buf.pop();
                            }
                        },
                    }// end single byte match
                }// end case SlowPath
            }// end match state
        }// end for each byte

        if self.cursor == last_cursor {
            None
        } else {
            let buf = 
                match (&t, &self.buffer_type) {
                      (&TokenType::String, &BufferType::Bytes)
                     |(&TokenType::Number, &BufferType::Bytes) => Buffer::MultiByte(buf),
                    _ => 
                        Buffer::Span(Span {
                                    first: first,
                                    end: self.cursor
                                }),
                };
            Some(Token {
                kind: t,
                buf: buf,
            })
        }
    }
}