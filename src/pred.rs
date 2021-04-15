use crate::common::{ParseResult, Parser};

fn any_char(input: &str) -> ParseResult<char> {
    match input.chars().next() {
        Some(next) => Ok((&input[next.len_utf8()..], next)),
        _ => Err(input)
    }
}

fn pred<'a, P, A, F>(p: P, f: F) -> impl Parser<'a, A>
    where P: Parser<'a, A>,
          F: Fn(&A) -> bool {
    |input| {
        match p.parse(input) {
            Ok((s, r)) => {
                if f(&r) {
                    return Ok((s, r));
                }
                Err(input)
            }
            Err(_) => Err(input)
        }
    }
}

#[test]
fn test_any_char() {
    let result = any_char("1osa");
    assert_eq!(Ok(("osa", '1')), result);
}