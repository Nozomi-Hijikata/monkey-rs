use lalrpop_util::lalrpop_mod;

lalrpop_mod!(parser);

#[test]
fn test_infix_expr() {
    let expr = parser::ExprParser::new().parse("1 + 2 * 3").unwrap();
    assert_eq!(format!("{:?}", expr), "(1 + (2 * 3))");

    let expr = parser::ExprParser::new().parse("1 * 2 + 3").unwrap();
    assert_eq!(format!("{:?}", expr), "((1 * 2) + 3)");

    let expr = parser::ExprParser::new().parse("1 + 2 + 3").unwrap();
    assert_eq!(format!("{:?}", expr), "((1 + 2) + 3)");

    let expr = parser::ExprParser::new().parse("1 * 2 * 3").unwrap();
    assert_eq!(format!("{:?}", expr), "((1 * 2) * 3)");

    let expr = parser::ExprParser::new().parse("1 + 2 * 3 + 4").unwrap();
    assert_eq!(format!("{:?}", expr), "((1 + (2 * 3)) + 4)");

    let expr = parser::ExprParser::new().parse("1 * 2 + 3 * 4").unwrap();
    assert_eq!(format!("{:?}", expr), "((1 * 2) + (3 * 4))");

    let expr = parser::ExprParser::new().parse("1 + 2 + 3 + 4").unwrap();
    assert_eq!(format!("{:?}", expr), "(((1 + 2) + 3) + 4)");

    let expr = parser::ExprParser::new().parse("1 * 2 * 3 * 4").unwrap();
    assert_eq!(format!("{:?}", expr), "(((1 * 2) * 3) * 4)");
}

#[test]
fn test_prefix_expr() {
    let expr = parser::ExprParser::new().parse("-1 + 2").unwrap();
    assert_eq!(format!("{:?}", expr), "((-1) + 2)");

    let expr = parser::ExprParser::new().parse("1 + -2").unwrap();
    assert_eq!(format!("{:?}", expr), "(1 + (-2))");

    let expr = parser::ExprParser::new().parse("-1 * 2").unwrap();
    assert_eq!(format!("{:?}", expr), "((-1) * 2)");

    let expr = parser::ExprParser::new().parse("-(1 + 2)").unwrap();
    assert_eq!(format!("{:?}", expr), "(-(1 + 2))");

    let expr = parser::ExprParser::new().parse("+1 + 2").unwrap();
    assert_eq!(format!("{:?}", expr), "((+1) + 2)");

    let expr = parser::ExprParser::new().parse("!1 + 2").unwrap();
    assert_eq!(format!("{:?}", expr), "((!1) + 2)");

    let expr = parser::ExprParser::new().parse("!(1 + 2)").unwrap();
    assert_eq!(format!("{:?}", expr), "(!(1 + 2))");

    let expr = parser::ExprParser::new().parse("1 + !2").unwrap();
    assert_eq!(format!("{:?}", expr), "(1 + (!2))");
}
