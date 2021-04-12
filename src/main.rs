mod parser;

fn main() {
    println!("the first letter is {:?}", parser::the_letter_a("a中国测试"));
    println!("the first letter is {:?}", parser::match_literal("Iam")("iama"));
}
