use crate::parser::{identifier, match_literal};

fn pair<P1, P2, R1, R2>(p1: P1, p2: P2) -> impl Fn(&str) -> Result<(&str, (R1, R2)), &str>
where
  P1: Fn(&str) -> Result<(&str, R1), &str>,
  P2: Fn(&str) -> Result<(&str, R2), &str>,
{
  move |input| {
    let res1 = p1(input);
      match res1 {
          Ok((s1, r1)) => {
              let res2 = p2(s1);
              match res2 {
                  Ok((s2, r2)) => Ok((s2, (r1, r2))),
                  Err(err) => Err(err)
              }
          }
          Err(err) => Err(err)
      }
  }
}

#[test]
fn test_pair() {
    let pair_fn = pair(match_literal("<"), identifier);
    let res = pair_fn("<I-am-test/>");
    assert_eq!(Ok(("/>", ((), "I-am-test".to_string()))), res);
}
