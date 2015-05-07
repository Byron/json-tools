use super::{Token, TokenType};
use std::collections::VecDeque;

pub struct FilterNull<I: Iterator<Item=Token>> {
    src: I,
    buf: VecDeque<Token>,
}

impl<I: Iterator<Item=Token>> FilterNull<I> {
    pub fn new(src: I) -> FilterNull<I> {
        FilterNull {
            src: src,
            buf: VecDeque::with_capacity(2),
        }
    }
}

impl<I> Iterator for FilterNull<I> where I: Iterator<Item=Token>{
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        if self.buf.len() > 0 {
            return self.buf.pop_front()
        }

        fn last_token(v: &mut VecDeque<Token>, t: Token) -> Option<Token> {
            // assume it's a comma
            if v.len() > 0 {
                v.push_back(t);
                v.pop_front()
            } else {
                Some(t)
            }
        }

        let token = self.src.next();
        match token {
            Some(mut first_str_candidate) => {
                loop {
                    match first_str_candidate.kind {
                        TokenType::String => {
                            let first_str_token = first_str_candidate;
                            match self.src.next() {
                                Some(colon_candidate) => {
                                    match colon_candidate.kind {
                                        TokenType::Colon => {
                                            let colon = colon_candidate;
                                            match self.src.next() {
                                                Some(second_str_candidate) => {
                                                    match second_str_candidate.kind {
                                                        TokenType::Null => {
                                                            // WE HAVE A STR : STR triplete, and we forget it
                                                            // This works by just not putting it onto the ringbuffer
                                                            // See if there is a (optional) comma
                                                            // If self.buf has anything, it must be commas !
                                                            // Usually, it is only 0 or 1 !
                                                            match self.src.next() {
                                                                Some(comma_candidate) => {
                                                                    first_str_candidate = 
                                                                        match match comma_candidate.kind {
                                                                            TokenType::Comma => self.src.next(),
                                                                            _ => {
                                                                                self.buf.pop_front();
                                                                                Some(comma_candidate)
                                                                            }
                                                                        } {
                                                                            Some(t) => t,
                                                                            None => return None,
                                                                        };
                                                                    continue;
                                                                },
                                                                None => return None,
                                                            }
                                                        },
                                                        _ => {
                                                            let res = last_token(&mut self.buf, first_str_token);
                                                            self.buf.push_back(colon);
                                                            self.buf.push_back(second_str_candidate);
                                                            return res
                                                        }
                                                    }
                                                },
                                                None => {
                                                    let res = last_token(&mut self.buf, first_str_token);
                                                    self.buf.push_back(colon);
                                                    return res
                                                }
                                            }
                                        },
                                        _ => {
                                            let res = last_token(&mut self.buf, first_str_token);
                                            self.buf.push_back(colon_candidate);
                                            return res
                                        }
                                    }// end is colon token
                                },// end have token (colon?)
                                None => return last_token(&mut self.buf, first_str_token),
                            }// end match next token (colon?)
                         }// end is string token,
                         TokenType::Comma => {
                            match self.src.next() {
                                None => return Some(first_str_candidate),
                                Some(t) => {
                                    // keep it, it will be returned first in case we 
                                    // end up not having a match
                                    // NOTE: will keep pushing back malformed ,,,,, sequences
                                    // which could be used for DOS attacks. TODO: impl. put_back
                                    self.buf.push_back(first_str_candidate);
                                    first_str_candidate = t;
                                    continue
                                }
                            }
                         },
                         _ => return last_token(&mut self.buf, first_str_candidate),
                     }// end match token kind (string?)
                }// end inner str candidate LOOP
            },// end have token
            None => None,
        }
    }
}

