mod ast;
use lalrpop_util::lalrpop_mod;


lalrpop_mod!(grammar);


fn main() {
    println!("Hello, world!");
}

#[test]
fn test() {
    let expr = grammar::ExprParser::new().parse("1 + 2 * 3").unwrap();
    assert_eq!(format!("{:?}", expr), "(1 + (2 * 3))");
}
