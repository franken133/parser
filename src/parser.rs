pub fn the_letter_a(input: &str) -> Result<(&str, ()), &str> {
    let mut chars = input.chars();
    if let Some('a') = chars.next() {
        return Ok((&input[1..], ()));
    }
    Err(input)
}

pub fn match_literal(expected: &'static str) -> impl Fn(&str) -> Result<(&str, ()), &str> {
    move |input: &str| match input.get(0..expected.len()) {
        Some(expected) => Ok((&input[expected.len()..], ())),
        None => Err(input)
    }
}