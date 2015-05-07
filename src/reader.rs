use std::io::{Read, Result, self};

use super::{Token, TokenType};

pub struct TokenReader<'a, I: Iterator<Item=Token>> {
    iter: I,
    src: Option<&'a str>,
}

impl<'a, I: Iterator<Item=Token>> TokenReader<'a, I> {
    pub fn new(iter: I, source: Option<&'a str>) -> TokenReader<'a, I> {
        TokenReader {
            iter: iter,
            src: source,
        }
    }
}

impl<'a, I: Iterator<Item=Token>> Read for TokenReader<'a, I> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        Err(io::Error::last_os_error())
    }
}