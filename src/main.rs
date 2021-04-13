mod parser;
mod pair;
mod common;

use crate::common::Parser;

fn main() {
    println!("the first letter is {:?}", parser::the_letter_a("a中国测试"));
    let match_fn = parser::match_literal("world");
    println!("the match_literal letter is {:?}", match_fn.parse("Iam"));
    println!("the identifier letter is {:?}", parser::identifier("Iam-test-identifi/>"));
}
