use crate::common::ParseResult;

fn any_char(input: &str) -> ParseResult<char> {
    match input.chars().next() {
        Some(next) => Ok((&input[next.len_utf8()..], next)),
        _ => Err(input)
    }
}

#[test]
fn test_any_char() {
    let result = any_char("1osa");
    assert_eq!(Ok(("osa", '1')), result);
}