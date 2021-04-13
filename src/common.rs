pub type ParseResult<'a, Output> = Result<(&'a str, Output), &'a str>;

pub trait Parser<'a, T>{
  fn parse(&self, input: &'a str) -> ParseResult<'a, T>;
}

impl<'a, F, T> Parser<'a, T> for F 
where F: Fn(&'a str) -> ParseResult<T>{
  fn parse(&self, input: &'a str) -> ParseResult<'a, T> {
    self(input)
  }
}