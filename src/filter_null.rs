use super::Token;

pub struct FilterNull<I: Iterator<Item=Token>> {
    src: I,
}

impl<I: Iterator<Item=Token>> FilterNull<I> {
    pub fn new(src: I) -> FilterNull<I> {
        FilterNull {
            src: src
        }
    }
}

impl<I> Iterator for FilterNull<I> where I: Iterator<Item=Token>{
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        None
    }
}

