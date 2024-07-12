use lalrpop_util::lalrpop_mod;

lalrpop_mod!(parser);

#[test]
fn test_integer() {
    let expr = parser::ExprParser::new().parse("1").unwrap();
    assert_eq!(format!("{:?}", expr), "1");

    let expr = parser::ExprParser::new().parse("123").unwrap();
    assert_eq!(format!("{:?}", expr), "123");
}

#[test]
fn test_identifier() {
    let expr = parser::ExprParser::new().parse("foobar").unwrap();
    assert_eq!(format!("{:?}", expr), "foobar");

    let expr = parser::ExprParser::new().parse("foo_bar").unwrap();
    assert_eq!(format!("{:?}", expr), "foo_bar");

    let expr = parser::ExprParser::new().parse("foo123").unwrap();
    assert_eq!(format!("{:?}", expr), "foo123");

    let expr = parser::ExprParser::new().parse("foo_bar123").unwrap();
    assert_eq!(format!("{:?}", expr), "foo_bar123");
}

#[test]
fn test_boolean() {
    let expr = parser::ExprParser::new().parse("true").unwrap();
    assert_eq!(format!("{:?}", expr), "true");

    let expr = parser::ExprParser::new().parse("false").unwrap();
    assert_eq!(format!("{:?}", expr), "false");
}

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
