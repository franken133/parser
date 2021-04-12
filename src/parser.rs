pub fn the_letter_a(input: &str) -> Result<(&str, ()), &str> {
    let mut chars = input.chars();
    if let Some('a') = chars.next() {
        return Ok((&input[1..], ()));
    }
    Err(input)
}

pub fn match_literal(expected: &'static str) -> impl Fn(&str) -> Result<(&str, ()), &str> {
    move |input: &str| match input.get(0..expected.len()) {
        Some(next) if next == expected => Ok((&input[expected.len()..], ())),
        _ => Err(input),
    }
}

pub fn identifier(input: &str) -> Result<(&str, String), &str> {
    let mut chars = input.chars();
    let mut result = String::from("");
    match chars.next() {
        Some(first) if first.is_alphabetic() => result.push(first),
        _ => return Err(input),
    }
    while let Some(matched) = chars.next() {
        if matched.is_alphanumeric() || matched == '-' {
            result.push(matched);
        } else {
            break;
        }
    }
    return Ok((&input[result.len()..], result));
}

pub fn is_whitespace() {
    
}

#[test]
fn test_match_literal() {
    let match_fn = match_literal("world");
    assert_eq!(Ok((" hello", ())), match_fn("world hello"));
    assert_eq!(Err("not world"), match_fn("not world"));
}

#[test]
fn test_identifier() {
    assert_eq!(Ok(("/>", "I-am-world".to_string())), identifier("I-am-world/>"));
    assert_eq!(Err(""), identifier(""));
    assert_eq!(Err("数字开头"), identifier("数字开头"));
}
