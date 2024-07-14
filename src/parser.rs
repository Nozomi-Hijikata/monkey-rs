use lalrpop_util::lalrpop_mod;

lalrpop_mod!(grammar);

#[test]
fn test_integer() {
    let expr = grammar::ExprParser::new().parse("1").unwrap();
    assert_eq!(format!("{:?}", expr), "1");

    let expr = grammar::ExprParser::new().parse("123").unwrap();
    assert_eq!(format!("{:?}", expr), "123");
}

#[test]
fn test_identifier() {
    let expr = grammar::ExprParser::new().parse("foobar").unwrap();
    assert_eq!(format!("{:?}", expr), "foobar");

    let expr = grammar::ExprParser::new().parse("foo_bar").unwrap();
    assert_eq!(format!("{:?}", expr), "foo_bar");

    let expr = grammar::ExprParser::new().parse("foo123").unwrap();
    assert_eq!(format!("{:?}", expr), "foo123");

    let expr = grammar::ExprParser::new().parse("foo_bar123").unwrap();
    assert_eq!(format!("{:?}", expr), "foo_bar123");
}

#[test]
fn test_boolean() {
    let expr = grammar::ExprParser::new().parse("true").unwrap();
    assert_eq!(format!("{:?}", expr), "true");

    let expr = grammar::ExprParser::new().parse("false").unwrap();
    assert_eq!(format!("{:?}", expr), "false");
}

#[test]
fn test_infix_expr() {
    let expr = grammar::ExprParser::new().parse("1+ 2 * 3").unwrap();
    assert_eq!(format!("{:?}", expr), "(1 + (2 * 3))");

    let expr = grammar::ExprParser::new().parse("1 * 2+ 3").unwrap();
    assert_eq!(format!("{:?}", expr), "((1 * 2) + 3)");

    let expr = grammar::ExprParser::new().parse("1 + 2+ 3").unwrap();
    assert_eq!(format!("{:?}", expr), "((1 + 2) + 3)");

    let expr = grammar::ExprParser::new().parse("1 *2 * 3").unwrap();
    assert_eq!(format!("{:?}", expr), "((1 * 2) * 3)");

    let expr = grammar::ExprParser::new().parse("1 + 2 * 3 + 4").unwrap();
    assert_eq!(format!("{:?}", expr), "((1 + (2 * 3)) + 4)");

    let expr = grammar::ExprParser::new().parse("1 * 2 + 3 * 4").unwrap();
    assert_eq!(format!("{:?}", expr), "((1 * 2) + (3 * 4))");

    let expr = grammar::ExprParser::new().parse("1 + 2 + 3 + 4").unwrap();
    assert_eq!(format!("{:?}", expr), "(((1 + 2) + 3) + 4)");

    let expr = grammar::ExprParser::new().parse("1 * 2 * 3 * 4").unwrap();
    assert_eq!(format!("{:?}", expr), "(((1 * 2) * 3) * 4)");
}

#[test]
fn test_prefix_expr() {
    let expr = grammar::ExprParser::new().parse("-1 + 2").unwrap();
    assert_eq!(format!("{:?}", expr), "((-1) + 2)");

    let expr = grammar::ExprParser::new().parse("1 + -2").unwrap();
    assert_eq!(format!("{:?}", expr), "(1 + (-2))");

    let expr = grammar::ExprParser::new().parse("-1 * 2").unwrap();
    assert_eq!(format!("{:?}", expr), "((-1) * 2)");

    let expr = grammar::ExprParser::new().parse("-(1 + 2)").unwrap();
    assert_eq!(format!("{:?}", expr), "(-(1 + 2))");

    let expr = grammar::ExprParser::new().parse("+1 + 2").unwrap();
    assert_eq!(format!("{:?}", expr), "((+1) + 2)");

    let expr = grammar::ExprParser::new().parse("!1 + 2").unwrap();
    assert_eq!(format!("{:?}", expr), "((!1) + 2)");

    let expr = grammar::ExprParser::new().parse("!(1 + 2)").unwrap();
    assert_eq!(format!("{:?}", expr), "(!(1 + 2))");

    let expr = grammar::ExprParser::new().parse("1 + !2").unwrap();
    assert_eq!(format!("{:?}", expr), "(1 + (!2))");
}

#[test]
fn test_if_expr() {
    let stmt = grammar::ExprParser::new().parse("if (true) { 1; }").unwrap();
    assert_eq!(format!("{:?}", stmt), "if (true) {\n  1\n}");

    let stmt = grammar::ExprParser::new().parse("if (true) { 1; } else { 2; }").unwrap();
    assert_eq!(format!("{:?}", stmt), "if (true) {\n  1\n} else {\n  2\n}");
}

#[test]
fn test_let_stmt() {
    let stmt = grammar::StmtParser::new().parse("let a = 1;").unwrap();
    assert_eq!(format!("{:?}", stmt), "let a = 1");

    let stmt = grammar::StmtParser::new().parse("let a = 1 + 2;").unwrap();
    assert_eq!(format!("{:?}", stmt), "let a = (1 + 2)");

    let stmt = grammar::StmtParser::new()
        .parse("let a = 1 + 2 * 3;")
        .unwrap();
    assert_eq!(format!("{:?}", stmt), "let a = (1 + (2 * 3))");

    let stmt = grammar::StmtParser::new()
        .parse("let a = 1 * 2 + 3;")
        .unwrap();
    assert_eq!(format!("{:?}", stmt), "let a = ((1 * 2) + 3)");

    let stmt = grammar::StmtParser::new()
        .parse("let a = 1 + 2 + 3;")
        .unwrap();
    assert_eq!(format!("{:?}", stmt), "let a = ((1 + 2) + 3)");

    let stmt = grammar::StmtParser::new()
        .parse("let a = 1 * 2 * 3;")
        .unwrap();
    assert_eq!(format!("{:?}", stmt), "let a = ((1 * 2) * 3)");

    let stmt = grammar::StmtParser::new()
        .parse("let a = 1 + 2 * 3 + 4;")
        .unwrap();
    assert_eq!(format!("{:?}", stmt), "let a = ((1 + (2 * 3)) + 4)");

    let stmt = grammar::StmtParser::new()
        .parse("let a = 1 * 2 + 3 * 4;")
        .unwrap();
    assert_eq!(format!("{:?}", stmt), "let a = ((1 * 2) + (3 * 4))");

    let stmt = grammar::StmtParser::new()
        .parse("let a = 1 + 2 + 3 + 4;")
        .unwrap();
    assert_eq!(format!("{:?}", stmt), "let a = (((1 + 2) + 3) + 4)");

    let stmt = grammar::StmtParser::new()
        .parse("let a = 1 * 2 * 3 * 4;")
        .unwrap();
    assert_eq!(format!("{:?}", stmt), "let a = (((1 * 2) * 3) * 4)");
}

#[test]
fn test_return_stmt() {
    let stmt = grammar::StmtParser::new().parse("return 1;").unwrap();
    assert_eq!(format!("{:?}", stmt), "return 1");

    let stmt = grammar::StmtParser::new().parse("return 1 + 2;").unwrap();
    assert_eq!(format!("{:?}", stmt), "return (1 + 2)");

    let stmt = grammar::StmtParser::new()
        .parse("return 1 + 2 * 3;")
        .unwrap();
    assert_eq!(format!("{:?}", stmt), "return (1 + (2 * 3))");

    let stmt = grammar::StmtParser::new()
        .parse("return 1 * 2 + 3;")
        .unwrap();
    assert_eq!(format!("{:?}", stmt), "return ((1 * 2) + 3)");

    let stmt = grammar::StmtParser::new()
        .parse("return 1 + 2 + 3;")
        .unwrap();
    assert_eq!(format!("{:?}", stmt), "return ((1 + 2) + 3)");

    let stmt = grammar::StmtParser::new()
        .parse("return 1 * 2 * 3;")
        .unwrap();
    assert_eq!(format!("{:?}", stmt), "return ((1 * 2) * 3)");

    let stmt = grammar::StmtParser::new()
        .parse("return 1 + 2 * 3 + 4;")
        .unwrap();
    assert_eq!(format!("{:?}", stmt), "return ((1 + (2 * 3)) + 4)");

    let stmt = grammar::StmtParser::new()
        .parse("return 1 * 2 + 3 * 4;")
        .unwrap();
    assert_eq!(format!("{:?}", stmt), "return ((1 * 2) + (3 * 4))");

    let stmt = grammar::StmtParser::new()
        .parse("return 1 + 2 + 3 + 4;")
        .unwrap();
    assert_eq!(format!("{:?}", stmt), "return (((1 + 2) + 3) + 4)");

    let stmt = grammar::StmtParser::new()
        .parse("return 1 * 2 * 3 * 4;")
        .unwrap();
    assert_eq!(format!("{:?}", stmt), "return (((1 * 2) * 3) * 4)");
}

#[test]
fn test_expr_stmt() {
    let stmt = grammar::StmtParser::new().parse("1;").unwrap();
    assert_eq!(format!("{:?}", stmt), "1");

    let stmt = grammar::StmtParser::new().parse("1 + 2;").unwrap();
    assert_eq!(format!("{:?}", stmt), "(1 + 2)");

    let stmt = grammar::StmtParser::new().parse("1 + 2 * 3;").unwrap();
    assert_eq!(format!("{:?}", stmt), "(1 + (2 * 3))");

    let stmt = grammar::StmtParser::new().parse("1 * 2 + 3;").unwrap();
    assert_eq!(format!("{:?}", stmt), "((1 * 2) + 3)");

    let stmt = grammar::StmtParser::new().parse("1 + 2 - 3;").unwrap();
    assert_eq!(format!("{:?}", stmt), "((1 + 2) - 3)");

    let stmt = grammar::StmtParser::new().parse("1 * 2 - 3 / 4;").unwrap();
    assert_eq!(format!("{:?}", stmt), "((1 * 2) - (3 / 4))");
}

#[test]
fn test_block_stmt() {
    let stmt = grammar::StmtParser::new().parse("{ 1; }").unwrap();
    assert_eq!(format!("{:?}", stmt), "{\n  1\n}");

    let stmt = grammar::StmtParser::new().parse("{ 1; 2; }").unwrap();
    assert_eq!(format!("{:?}", stmt), "{\n  1\n  2\n}");

    let stmt = grammar::StmtParser::new().parse("{ 1+2; 2*3; }").unwrap();
    assert_eq!(format!("{:?}", stmt), "{\n  (1 + 2)\n  (2 * 3)\n}");
}
