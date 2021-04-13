use crate::parser::{identifier, match_literal};
use crate::common::*;

fn pair<'a, P1, P2, R1, R2>(p1: P1, p2: P2) -> impl Parser<'a, (R1, R2)>
where
  P1: Parser<'a, R1>,
  P2: Parser<'a, R2>,
{
  move |input| {
    let res1 = p1.parse(input);
    match res1 {
      Ok((s1, r1)) => {
        let res2 = p2.parse(s1);
        match res2 {
          Ok((s2, r2)) => Ok((s2, (r1, r2))),
          Err(err) => Err(err),
        }
      }
      Err(err) => Err(err),
    }
  }
}

fn map<'a, P, F, A, B>(p: P, f: F) -> impl Parser<'a, B>
where
  P: Parser<'a, A>,
  F: Fn(A) -> B,
{
  move |input: &'a str| match p.parse(input) {
    Ok((s1, r1)) => Ok((s1, f(r1))),
    Err(_) => Err(input),
  }
}

#[test]
fn test_map() {
  let map_fn = map(pair(match_literal("<"), identifier), |(_, r2)| (r2));
  assert_eq!(Ok(("/>", "I-am-test".to_string())), map_fn.parse("<I-am-test/>"))
}

#[test]
fn test_pair() {
  let pair_fn = pair(match_literal("<"), identifier);
  let res = pair_fn.parse("<I-am-test/>");
  assert_eq!(Ok(("/>", ((), "I-am-test".to_string()))), res);
}
