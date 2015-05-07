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
                                                            match self.src.next() {
                                                                Some(comma_candidate) => {
                                                                    first_str_candidate = 
                                                                        match match comma_candidate.kind {
                                                                            TokenType::Comma => self.src.next(),
                                                                            _ => Some(comma_candidate)
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
                                                            self.buf.push_back(colon);
                                                            self.buf.push_back(second_str_candidate);
                                                            return Some(first_str_token)
                                                        }
                                                    }
                                                },
                                                None => {
                                                    self.buf.push_back(colon);
                                                    return Some(first_str_token)
                                                }
                                            }
                                        },
                                        _ => {
                                            self.buf.push_back(colon_candidate);
                                            return Some(first_str_token)
                                        }
                                    }// end is colon token
                                },// end have token (colon?)
                                None => return Some(first_str_token),
                            }// end match next token (colon?)
                         }// end is string token,
                         _ => return Some(first_str_candidate),
                     }// end match token kind (string?)
                }// end inner str candidate loop
            },// end have token
            None => None,
        }
    }
}

