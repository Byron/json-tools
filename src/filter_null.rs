use super::{Token, TokenType};
use std::collections::VecDeque;

pub struct FilterNull<I: Iterator<Item=Token>> {
    src: I,
    // NOTE: We could remove the deck and keep the 3 slots we need as Option<Token>
    // 0: optional comma
    // 1: first string
    // 2: colon
    buf: VecDeque<Token>,
    next_token: Option<Token>
}

impl<I: Iterator<Item=Token>> FilterNull<I> {
    pub fn new(src: I) -> FilterNull<I> {
        FilterNull {
            src: src,
            buf: VecDeque::with_capacity(3),
            next_token: None
        }
    }

    fn put_back(&mut self, t: Token) {
        debug_assert!(self.next_token.is_none());
        self.next_token = Some(t);
    }

    fn next_token(&mut self) -> Option<Token> {
        match self.next_token.take() {
            Some(t) => Some(t),
            None => self.src.next()
        }
    }
}

impl<I> Iterator for FilterNull<I> where I: Iterator<Item=Token>{
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        if self.buf.len() > 0 {
            return self.buf.pop_front()
        }

        fn first_token(v: &mut VecDeque<Token>, t: Token) -> Option<Token> {
            // assume it's a comma
            if v.len() > 0 {
                v.push_back(t);
                v.pop_front()
            } else {
                Some(t)
            }
        }

        let token = self.next_token();
        match token {
            Some(mut first_str_candidate) => {
                loop {
                    match first_str_candidate.kind {
                        TokenType::String => {
                            let first_str_token = first_str_candidate;
                            match self.next_token() {
                                Some(colon_candidate) => {
                                    match colon_candidate.kind {
                                        TokenType::Colon => {
                                            let colon = colon_candidate;
                                            match self.next_token() {
                                                Some(second_str_candidate) => {
                                                    match second_str_candidate.kind {
                                                        TokenType::Null => {
                                                            // WE HAVE A STR : STR triplete, and we forget it
                                                            // This works by just not putting it onto the ringbuffer
                                                            // See if there is a (optional) comma
                                                            // If self.buf has anything, it must be commas !
                                                            // Usually, it is only 0 or 1 !
                                                            match self.next_token() {
                                                                Some(comma_candidate) => {
                                                                    first_str_candidate = 
                                                                        match match comma_candidate.kind {
                                                                            TokenType::Comma => self.next_token(),
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
                                                            let res = first_token(&mut self.buf, first_str_token);
                                                            self.buf.push_back(colon);
                                                            self.buf.push_back(second_str_candidate);
                                                            return res
                                                        }
                                                    }
                                                },
                                                None => {
                                                    let res = first_token(&mut self.buf, first_str_token);
                                                    self.buf.push_back(colon);
                                                    return res
                                                }
                                            }
                                        },
                                        _ => {
                                            let res = first_token(&mut self.buf, first_str_token);
                                            self.buf.push_back(colon_candidate);
                                            return res
                                        }
                                    }// end is colon token
                                },// end have token (colon?)
                                None => return first_token(&mut self.buf, first_str_token),
                            }// end match next token (colon?)
                         }// end is string token,
                         TokenType::Comma => {
                            // NOTE: in case of malformed ,,,,, sequences, we just consider
                            // this a peek, return the previous comma, and put back this one
                            if self.buf.len() > 0 {
                                debug_assert_eq!(self.buf.len(), 1);
                                self.put_back(first_str_candidate);
                                return self.buf.pop_front()
                            }
                            match self.next_token() {
                                None => return Some(first_str_candidate),
                                Some(t) => {
                                    // keep it, it will be returned first in case we 
                                    // end up not having a match
                                    self.buf.push_back(first_str_candidate);
                                    first_str_candidate = t;
                                    continue
                                }
                            }
                         },
                         _ => return first_token(&mut self.buf, first_str_candidate),
                     }// end match token kind (string?)
                }// end inner str candidate LOOP
            },// end have token
            None => None,
        }
    }
}

