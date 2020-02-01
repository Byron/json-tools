use super::{FilterTypedKeyValuePairs, Token, TokenReader, TokenType};

/// Applies convenience constructors to all `Iterator<Item=Token>` types
pub trait IteratorExt: Iterator<Item = Token> {
    /// Returns an Iterator which filters key=value pairs, if `value.kind` matches
    /// the given `token_type`.
    ///
    /// It is useful, for example, to get rid of `null` values on a lexical level.
    fn filter_key_value_by_type(self, token_type: TokenType) -> FilterTypedKeyValuePairs<Self>
    where
        Self: Sized,
    {
        FilterTypedKeyValuePairs::new(self, token_type)
    }

    /// Returns a `TokenReader` to produce a byte stream from `Token` instances
    ///
    /// # Arguments
    /// * `source` - an optional, original string from which the tokens were
    ///              generated. This offers the best performance when
    ///              serializing tokens, as they can refer to their original
    ///              `&str` slice.
    fn reader(self, source: Option<&str>) -> TokenReader<Self>
    where
        Self: Sized,
    {
        TokenReader::new(self, source)
    }
}

impl<T: Iterator<Item = Token>> IteratorExt for T {}
