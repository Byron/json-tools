use std::{
    cmp,
    io::{Read, Result, Write},
    ptr,
};

use super::{Buffer, Token, TokenType};

fn copy_memory(src: &[u8], dst: &mut [u8]) {
    let len_src = src.len();
    assert!(dst.len() >= len_src);
    // `dst` is unaliasable, so we know statically it doesn't overlap
    // with `src`.
    unsafe {
        ptr::copy_nonoverlapping(src.as_ptr(), dst.as_mut_ptr(), len_src);
    }
}

/// An adapter to convert a stream of `Token`s into bytes by implementing an
/// `std::io::Read` trait.
///
/// Currently it produces output without any whitespace, suitable for efficient
/// sending and for handling by machines.
pub struct TokenReader<'a, I: IntoIterator<Item = Token>> {
    iter: I::IntoIter,
    src: Option<&'a str>,
    buf: Vec<u8>,
    ofs: usize,
}

impl<'a, I: IntoIterator<Item = Token>> TokenReader<'a, I> {
    /// Returns a new `TokenReader`
    /// # Args
    /// * `iter` - the iterator producing `Token` instances we are to convert
    /// * `source` - an optional, original string from which the tokens were
    ///              generated. This offers the best performance when
    ///              serializing tokens, as they can refer to their original
    ///              `&str` slice.
    pub fn new(iter: I, source: Option<&'a str>) -> TokenReader<'a, I> {
        TokenReader {
            iter: iter.into_iter(),
            src: source,
            buf: Vec::with_capacity(128),
            ofs: 0,
        }
    }
}

impl<'a, I: IntoIterator<Item = Token>> Read for TokenReader<'a, I> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        if buf.is_empty() {
            return Ok(0);
        }

        // Bytes from Cache
        let mut bl = buf.len();
        if !self.buf.is_empty() {
            let btc = cmp::min(self.buf.len() - self.ofs, buf.len());
            copy_memory(&self.buf[self.ofs..self.ofs + btc], buf);
            bl -= btc;
            self.ofs += btc;
            if self.ofs == self.buf.len() {
                self.buf.clear();
                self.ofs = 0;
            }
        }
        if bl == 0 {
            return Ok(buf.len());
        }

        // Generate bytes from tokens
        while bl > 0 {
            match self.iter.next() {
                None => return Ok(buf.len() - bl),
                Some(t) => {
                    let bytes: &[u8] = match t.kind {
                        TokenType::String | TokenType::Number => match t.buf {
                            Buffer::MultiByte(ref b) => &b,
                            Buffer::Span(ref s) => match self.src {
                                Some(b) => b[s.first as usize..s.end as usize].as_bytes(),
                                None => panic!("Must set source if tokens don't provide byter buffers"),
                            },
                        },
                        TokenType::Invalid => b"",
                        _ => t.kind.as_ref().as_bytes(),
                    };
                    let btc = cmp::min(bytes.len(), bl);
                    let ofs = buf.len() - bl;
                    copy_memory(&bytes[..btc], &mut buf[ofs..ofs + btc]);
                    bl -= btc;

                    if btc < bytes.len() {
                        debug_assert!(bl == 0);
                        self.buf.write_all(&bytes[btc..])?;
                    }

                    if bl == 0 {
                        return Ok(buf.len());
                    }
                }
            } // match iter.next()
        } // end while there are bytes to produce
        unreachable!();
    }
}
