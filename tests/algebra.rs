use auto_diff::Variable;

#[test]
fn test_operations() {
    let x = Variable::Const(2);
    let y = Variable::Var("var", 10);
    assert_eq!(Variable::Var("var", 34), x * 2 + 3 * y);
}
