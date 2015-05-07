use std::io::{Read, Result, self};
use std::slice::bytes::copy_memory;
use std::cmp;

use super::{Token, TokenType, Buffer};

pub struct TokenReader<'a, I: Iterator<Item=Token>> {
    iter: I,
    src: Option<&'a str>,
    buf: Vec<u8>
}

impl<'a, I: Iterator<Item=Token>> TokenReader<'a, I> {
    pub fn new(iter: I, source: Option<&'a str>) -> TokenReader<'a, I> {
        TokenReader {
            iter: iter,
            src: source,
            buf: Vec::with_capacity(128)
        }
    }
}

impl<'a, I: Iterator<Item=Token>> Read for TokenReader<'a, I> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        if buf.len() == 0 {
            return Ok(0)
        }

        // Bytes from Cache
        let mut bl = buf.len();
        if self.buf.len() > 0 {
            let btc = cmp::min(self.buf.len(), buf.len());
            let new_buf = self.buf.split_off(btc);
            copy_memory(&self.buf, buf);
            self.buf = new_buf;
            bl -= btc;
        }
        if bl == 0 {
            return Ok(buf.len());
        }

        // Generate bytes from tokens
        while bl > 0 {
            match self.iter.next() {
                None => {
                    // if we have not read any byte yet, we may return an error
                    if bl == buf.len() {
                        return Err(io::Error::new(io::ErrorKind::Other, "End of Token-Iterator"))
                    } else {
                        return Ok(buf.len() - bl)
                    }
                },
                Some(t) => {
                    let bytes: &[u8] = 
                        match t.kind {
                             TokenType::String
                            |TokenType::Number => {
                                match t.buf {
                                    Buffer::MultiByte(ref b) => &b,
                                    Buffer::Span(ref s) => match self.src {
                                        Some(b) => b[s.first as usize .. s.end as usize].as_bytes(),
                                        None => panic!("Must set source if tokens don't provide byter buffers"),
                                    }
                                }
                            },
                            TokenType::Invalid => "".as_bytes(), // TODO: What to do ? Configure ?
                            _ => t.kind.as_ref().as_bytes(),
                        };
                    let btc = cmp::min(bytes.len(), bl);
                    copy_memory(&bytes[..btc], buf);
                    bl -= btc;

                    if btc < bytes.len() {
                        debug_assert!(bl == 0);
                        self.buf.push_all(&bytes[btc..])
                    }

                    if bl == 0 {
                        return Ok(buf.len())
                    }
                }
            }// match iter.next()
        }// end while there are bytes to produce
        unreachable!();
    }
}