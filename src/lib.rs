/// A lexer for utf-8 encoded json data
pub struct Lexer<I> 
                where I: Iterator<Item=char> {
    chars: I,
    cursor: u64,
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    /// All whitespace
    WhiteSpace,

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
    /// Index of the first the character
    pub first: u64,
    /// Index one past the last character
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
            cursor: 0,
        }
    }
}

impl<I> Iterator for Lexer<I> 
                    where I: Iterator<Item=char> {
    type Item = Token;

    /// Lex the underlying character stream to generate tokens
    fn next(&mut self) -> Option<Token> {
        None
    }
}