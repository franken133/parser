mod parser;
mod pair;

fn main() {
    println!("the first letter is {:?}", parser::the_letter_a("a中国测试"));
    println!("the match_literal letter is {:?}", parser::match_literal("Iam")("iama"));
    println!("the identifier letter is {:?}", parser::identifier("Iam-test-identifi/>"));
}
