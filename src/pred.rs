use crate::common::{ParseResult, Parser};
use crate::pair::{zero_or_more, one_or_more, pair, right, left, map};
use crate::parser::{match_literal, identifier};

fn any_char(input: &str) -> ParseResult<char> {
    match input.chars().next() {
        Some(next) => Ok((&input[next.len_utf8()..], next)),
        _ => Err(input)
    }
}

fn pred<'a, P, A, F>(p: P, f: F) -> impl Parser<'a, A>
    where P: Parser<'a, A>,
          F: Fn(&A) -> bool {
    move |input| {
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

fn space1<'a>() -> impl Parser<'a, Vec<char>>{
    one_or_more(whitespace_char())
}

fn space0<'a>() -> impl Parser<'a, Vec<char>>{
    zero_or_more(whitespace_char())
}

fn whitespace_char<'a>() -> impl Parser<'a, char> {
    pred(any_char, |c| c.is_whitespace())
}

fn quoted_string<'a>() -> impl Parser<'a, String> {
    let zero_fn = left(zero_or_more(pred(any_char, |c| *c != '"')), match_literal("\""));
    // let zero_fn = right(match_literal("\""), zero_fn);
    map(right(match_literal("\""), zero_fn), |chars|chars.into_iter().collect())
}

#[test]
fn test_quoted_string() {
    let string_fn = quoted_string().parse("\"hello=world\"");
    println!("result is {:?}", string_fn);
}

#[test]
fn test_space0() {
    let result = space0().parse("1asb");
    assert_eq!(Ok(("1asb", vec![])), result);
    let result = space0().parse("   as");
    assert_eq!(Ok(("as", vec![' ', ' ', ' '])), result);
}

#[test]
fn test_space1() {
    let result = space1().parse("1asb");
    assert_eq!(Err("1asb"), result);
    let result = space1().parse("  1asb");
    assert_eq!(Ok(("1asb", vec![' ', ' '])), result);
}

#[test]
fn test_any_char() {
    let result = any_char("1osa");
    assert_eq!(Ok(("osa", '1')), result);
}

#[test]
fn test_whitespace_char() {
    let white_fn = whitespace_char();
    assert_eq!(Ok(("a", ' ')), white_fn.parse(" a"));
    assert_eq!(Err("1 a"), white_fn.parse("1 a"));
}