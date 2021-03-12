use rust_quantifier_elimination::parse;

fn main() {
    println!("{:?}", parse("123 + 456 + x").unwrap());
}
