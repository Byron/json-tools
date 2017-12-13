/// A lexer for utf-8 encoded json data
pub struct Lexer<I: IntoIterator<Item = u8>> {
    chars: I::IntoIter,
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

impl AsRef<str> for TokenType {
    fn as_ref(&self) -> &str {
        match *self {
            TokenType::CurlyOpen => "{",
            TokenType::CurlyClose => "}",
            TokenType::BracketOpen => "[",
            TokenType::BracketClose => "]",
            TokenType::Colon => ":",
            TokenType::Comma => ",",
            TokenType::BooleanTrue => "true",
            TokenType::BooleanFalse => "false",
            TokenType::Null => "null",

            TokenType::Invalid => panic!("Cannot convert invalid TokenType"),
            _ => panic!("Cannot convert variant TokenTypes"),
        }
    }
}

/// A pair of indices into the byte stream returned by our source
/// iterator.
/// It is an exclusive range.
#[derive(Debug, PartialEq, Clone, Default)]
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
    pub buf: Buffer,
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
#[derive(Debug, PartialEq, Clone)]
pub enum BufferType {
    /// Use a `Buffer::MultiByte` were appropriate. Initialize it with the
    /// given capcity (to obtain higher performance when pushing charcters)
    Bytes(usize),
    Span,
}

impl<I> Lexer<I>
where
    I: IntoIterator<Item = u8>,
{
    /// Returns a new Lexer from a given byte iterator.
    pub fn new(chars: I, buffer_type: BufferType) -> Lexer<I> {
        Lexer {
            chars: chars.into_iter(),
            next_byte: None,
            cursor: 0,
            buffer_type: buffer_type,
        }
    }

    pub fn into_inner(self) -> I::IntoIter {
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
            }
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
where
    I: IntoIterator<Item = u8>,
{
    type Item = Token;

    /// Lex the underlying bytte stream to generate tokens
    fn next(&mut self) -> Option<Token> {
        let mut t: Option<TokenType> = None;

        let mut first = 0;
        let mut state = Mode::SlowPath;
        let last_cursor = self.cursor;
        let mut buf = match self.buffer_type {
            BufferType::Bytes(capacity) => Some(Vec::<u8>::with_capacity(capacity)),
            BufferType::Span => None,
        };

        while let Some(c) = self.next_byte() {
            let mut set_cursor = |cursor| {
                first = cursor - 1;
            };

            match state {
                Mode::String(ref mut ign_next) => {
                    if let Some(ref mut v) = buf {
                        v.push(c);
                    }
                    if *ign_next && (c == b'"' || c == b'\\') {
                        *ign_next = false;
                        continue;
                    }
                    match c {
                        b'"' => {
                            t = Some(TokenType::String);
                            break;
                        }
                        b'\\' => {
                            *ign_next = true;
                            continue;
                        }
                        _ => {
                            continue;
                        }
                    }
                }
                Mode::Null(ref mut b, ref mut i) => {
                    b[*i] = c;
                    if *i == 3 {
                        // we know b[0] is b'n'
                        if b[1] == b'u' && b[2] == b'l' && b[3] == b'l' {
                            t = Some(TokenType::Null);
                        } else {
                            t = Some(TokenType::Invalid);
                        }
                        break;
                    } else {
                        *i += 1;
                        continue;
                    }
                }
                Mode::Number => match c {
                    b'0'...b'9' | b'-' | b'.' => {
                        if let Some(ref mut v) = buf {
                            v.push(c);
                        }
                        continue;
                    }
                    _ => {
                        t = Some(TokenType::Number);
                        self.put_back(c);
                        break;
                    }
                },
                Mode::True(ref mut b, ref mut i) => {
                    b[*i] = c;
                    if *i == 3 {
                        // we know b[0] is b't'
                        if b[1] == b'r' && b[2] == b'u' && b[3] == b'e' {
                            t = Some(TokenType::BooleanTrue);
                        } else {
                            t = Some(TokenType::Invalid);
                        }
                        break;
                    } else {
                        *i += 1;
                        continue;
                    }
                }
                Mode::False(ref mut b, ref mut i) => {
                    b[*i] = c;
                    if *i == 4 {
                        // we know b[0] is b'f'
                        if b[1] == b'a' && b[2] == b'l' && b[3] == b's' && b[4] == b'e' {
                            t = Some(TokenType::BooleanFalse);
                        } else {
                            t = Some(TokenType::Invalid);
                        }
                        break;
                    } else {
                        *i += 1;
                        continue;
                    }
                }
                Mode::SlowPath => {
                    match c {
                        b'{' => {
                            t = Some(TokenType::CurlyOpen);
                            set_cursor(self.cursor);
                            break;
                        }
                        b'}' => {
                            t = Some(TokenType::CurlyClose);
                            set_cursor(self.cursor);
                            break;
                        }
                        b'"' => {
                            state = Mode::String(false);
                            if let Some(ref mut v) = buf {
                                v.push(c);
                            } else {
                                set_cursor(self.cursor);
                                // it starts at invalid, and once we know it closes, it's a string
                                t = Some(TokenType::Invalid);
                            }
                        }
                        b'n' => {
                            state = Mode::Null([c, b'x', b'x', b'x'], 1);
                            set_cursor(self.cursor);
                        }
                        b'0'...b'9' | b'-' | b'.' => {
                            state = Mode::Number;
                            if let Some(ref mut v) = buf {
                                v.push(c);
                            } else {
                                set_cursor(self.cursor);
                            }
                        }
                        b't' => {
                            state = Mode::True([c, b'x', b'x', b'x'], 1);
                            set_cursor(self.cursor);
                        }
                        b'f' => {
                            state = Mode::False([c, b'x', b'x', b'x', b'x'], 1);
                            set_cursor(self.cursor);
                        }
                        b'[' => {
                            t = Some(TokenType::BracketOpen);
                            set_cursor(self.cursor);
                            break;
                        }
                        b']' => {
                            t = Some(TokenType::BracketClose);
                            set_cursor(self.cursor);
                            break;
                        }
                        b':' => {
                            t = Some(TokenType::Colon);
                            set_cursor(self.cursor);
                            break;
                        }
                        b',' => {
                            t = Some(TokenType::Comma);
                            set_cursor(self.cursor);
                            break;
                        }
                        b'\\' => {
                            // invalid
                            t = Some(TokenType::Invalid);
                            set_cursor(self.cursor);
                            break;
                        }
                        _ => {}
                    } // end single byte match
                } // end case SlowPath
            } // end match state
        } // end for each byte

        match t {
            None => None,
            Some(t) => {
                if self.cursor == last_cursor {
                    None
                } else {
                    let buf = match (&t, buf) {
                        (&TokenType::String, Some(b)) | (&TokenType::Number, Some(b)) => Buffer::MultiByte(b),
                        _ => Buffer::Span(Span {
                            first: first,
                            end: self.cursor,
                        }),
                    };
                    Some(Token { kind: t, buf: buf })
                }
            }
        }
    }
}
