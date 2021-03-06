use crate::common::*;
use crate::parser::{identifier, match_literal};

pub fn pair<'a, P1, P2, R1, R2>(p1: P1, p2: P2) -> impl Parser<'a, (R1, R2)>
where
  P1: Parser<'a, R1>,
  P2: Parser<'a, R2>,
{
  move |input| {
    p1.parse(input)
      .and_then(|(s1, r1)| p2.parse(s1).map(|(s2, r2)| (s2, (r1, r2))))
    // match res1 {
    //   Ok((s1, r1)) => {
    //     let res2 = p2.parse(s1);
    //     match res2 {
    //       Ok((s2, r2)) => Ok((s2, (r1, r2))),
    //       Err(err) => Err(err),
    //     }
    //   }
    //   Err(err) => Err(err),
    // }
  }
}

pub fn map<'a, P, F, A, B>(p: P, f: F) -> impl Parser<'a, B>
where
  P: Parser<'a, A>,
  F: Fn(A) -> B,
{
  move |input: &'a str| p.parse(input).map(|(s1, r1)| (s1, f(r1)))
}

pub fn left<'a, P1, P2, R1, R2>(p1: P1, p2: P2) -> impl Parser<'a, R1>
where
  P1: Parser<'a, R1>,
  P2: Parser<'a, R2>,
{
  map(pair(p1, p2), |(r1, _)| r1)
  // let pair_fn = pair(p1, p2);
  // move |input| pair_fn.parse(input).map(|(s1, (r1, _))| (s1, r1))
}

pub fn right<'a, P1, P2, R1, R2>(p1: P1, p2: P2) -> impl Parser<'a, R2>
where
  P1: Parser<'a, R1>,
  P2: Parser<'a, R2>,
{
  map(pair(p1, p2), |(_, r2)| r2)
  // let pair_fn = pair(p1, p2);
  // move |input| pair_fn.parse(input).map(|(s1, (r1, _))| (s1, r1))
}

pub fn one_or_more<'a, P, A>(p: P) -> impl Parser<'a, Vec<A>>
where
  P: Parser<'a, A>,
{
  move |mut input| {
    let mut result = vec![];
    match p.parse(input) {
      Ok((s1, r1)) => {
        input = s1;
        result.push(r1);
      }
      Err(_) => {return Err(input)}
    }
    while let Ok((s1, r1)) = p.parse(input) {
      input = s1;
      result.push(r1);
    }
    Ok((input, result))
  }
}

pub fn zero_or_more<'a, P, A>(p: P) -> impl Parser<'a, Vec<A>>
where P: Parser<'a, A>
{
  move |mut input| {
    let mut result = vec![];
    while let Ok((s1, r1)) = p.parse(input) {
      input = s1;
      result.push(r1);
    }
    Ok((input, result))
  }
}

#[test]
fn test_right() {
  let right_fn = right(match_literal("<"), identifier);
  assert_eq!(
    Ok(("/>", "I-am-test".to_string())),
    right_fn.parse("<I-am-test/>")
  )
}

#[test]
fn test_zero_or_more() {
  let zero_fn = right(match_literal("<"), identifier);
  println!("result is {:?}", zero_fn.parse(r#"<single-element attribute="value" />"#));
}

#[test]
fn test_map() {
  let map_fn = map(pair(match_literal("<"), identifier), |(_, r2)| (r2));
  assert_eq!(
    Ok(("/>", "I-am-test".to_string())),
    map_fn.parse("<I-am-test/>")
  )
}

#[test]
fn test_pair() {
  let pair_fn = pair(match_literal("<"), identifier);
  let res = pair_fn.parse("<I-am-test/>");
  assert_eq!(Ok(("/>", ((), "I-am-test".to_string()))), res);
}
