use rust_quantifier_elimination::parse;

fn main() {
    println!("{:?}", parse("(E x) 123 + 456 + x > x^2").unwrap());
}
