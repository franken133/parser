pub type ParseResult<'a, Output> = Result<(&'a str, Output), &'a str>;

pub trait Parser<'a, T> {
    fn parse(&self, input: &'a str) -> ParseResult<'a, T>;
}

impl<'a, F, T> Parser<'a, T> for F
where
    F: Fn(&'a str) -> ParseResult<T>,
{
    fn parse(&self, input: &'a str) -> ParseResult<'a, T> {
        self(input)
    }
}

pub struct BoxedParser<'a, Output> {
    parser: Box<dyn Parser<'a, Output> + 'a>,
}

impl<'a, Output> BoxedParser<'a, Output> {
    fn new<P>(p: P) -> Self
    where
        P: Parser<'a, Output> + 'a,
    {
        BoxedParser {
            parser: Box::new(p),
        }
    }
}

impl<'a, T> Parser<'a, T> for BoxedParser<'a, T> {
    fn parse(&self, input: &'a str) -> ParseResult<'a, T> {
        self.parser.parse(input)
    }
}
